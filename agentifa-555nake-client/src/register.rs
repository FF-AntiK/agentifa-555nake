use bevy::{
    core_pipeline::ClearColor,
    input::Input,
    math::Size,
    prelude::{
        App, BuildChildren, Color, Commands, Component, DespawnRecursiveExt, Entity, EventReader,
        KeyCode, NodeBundle, ParallelSystemDescriptorCoercion, Plugin, Query, Res, ResMut, State,
        SystemSet, TextBundle, UiCameraBundle, With,
    },
    text::{HorizontalAlign, Text, TextAlignment, TextStyle, VerticalAlign},
    ui::{AlignItems, JustifyContent, Style, Val},
    window::ReceivedCharacter,
};

use crate::{
    vkeyboard::{Button, Key},
    AppState, FontAssets, InputState, Player,
};

const FNTSZE: f32 = 30.0;
const NAMESZE: usize = 30;

pub struct RegisterPlugin;
impl Plugin for RegisterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Register).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Register).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Register)
                    .with_system(input_keyboard.after(InputState::Keyboard))
                    .with_system(input_vkeyboard)
                    .with_system(update_text),
            );
    }
}

#[derive(Component)]
struct RegisterComponent;

#[derive(Component)]
struct TextInput;

fn cleanup(mut commands: Commands, query: Query<Entity, With<RegisterComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn input_keyboard(
    mut app_state: ResMut<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    mut input_char: EventReader<ReceivedCharacter>,
    input_state: Res<InputState>,
    mut player: ResMut<Player>,
) {
    if !vec![InputState::Keyboard].contains(&input_state) {
        return;
    }

    let name = &mut player.name;
    if input.pressed(KeyCode::Back) {
        input.release(KeyCode::Back);

        if name.len() > 0 {
            name.pop().unwrap();
        }

        return;
    }

    if input.pressed(KeyCode::Return) {
        input.release(KeyCode::Return);
        app_state.pop().unwrap();
        return;
    }

    for e in input_char.iter() {
        if !e.char.is_control() && name.len() <= NAMESZE {
            name.push(e.char);
        }
    }
}

fn input_vkeyboard(
    mut app_state: ResMut<State<AppState>>,
    mut event_reader: EventReader<Button>,
    mut player: ResMut<Player>,
) {
    for btn in event_reader.iter() {
        let name = &mut player.name;

        match btn.key {
            Key::Backspace => {
                if name.len() > 0 {
                    name.pop().unwrap();
                }
            }
            Key::Return => {
                app_state.pop().unwrap();
            }
            _ => name.push_str(btn.to_string().as_str()),
        }
    }
}

fn setup(mut clear: ResMut<ClearColor>, mut commands: Commands, fonts: Res<FontAssets>) {
    clear.0 = Color::BLACK;
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(RegisterComponent);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                align_items: AlignItems::Center,
                flex_direction: bevy::ui::FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Enter your Agent ID:",
                    TextStyle {
                        color: Color::CYAN,
                        font: fonts.regular.clone(),
                        font_size: FNTSZE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });

            p.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "",
                    TextStyle {
                        color: Color::PINK,
                        font: fonts.regular.clone(),
                        font_size: FNTSZE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(TextInput);
        })
        .insert(RegisterComponent);
}

fn update_text(player: Res<Player>, mut text: Query<&mut Text, With<TextInput>>) {
    let mut txt = text.iter_mut().next().unwrap();
    let mut lbl = player.name.clone();

    if lbl.len() <= 0 {
        lbl = " ".to_string();
    }

    txt.sections[0].value = lbl;
}
