use std::fmt;

use bevy::{
    core_pipeline::ClearColor,
    input::Input,
    math::{Rect, Size, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        App, BuildChildren, ButtonBundle, ChildBuilder, Children, Color, Commands, Component,
        DespawnRecursiveExt, Entity, EventReader, EventWriter, GlobalTransform, Handle, KeyCode,
        MouseButton, NodeBundle, OrthographicCameraBundle, ParallelSystemDescriptorCoercion,
        Plugin, Query, Res, ResMut, State, SystemSet, TextBundle, Transform, UiCameraBundle, With,
    },
    sprite::{Sprite, SpriteBundle},
    text::{Font, Text, TextStyle},
    ui::{AlignItems, FlexDirection, Interaction, JustifyContent, Style, UiColor, Val},
    window::{WindowMode, Windows},
};
use bevy_kira_audio::Audio;
use rand::random;

use crate::{AppState, AudioAssets, FontAssets, ImageAssets, InputState};

#[cfg(not(target_arch = "wasm32"))]
use bevy::app::AppExit;

const BG_CLR: Color = Color::rgba(1., 1., 1., 0.1);
const BRD_BRD: f32 = 8.0; // Px
const BRD_CLR: Color = Color::rgb(0.65, 0.65, 0.65);
const BRD_SZE: f32 = 400.0; // Px
const BTN_CLR: Color = Color::rgb(0.15, 0.15, 0.15);
const BTN_CLR_HOV: Color = Color::rgb(1.0, 1.0, 1.0);
const BTN_TXT_CLR: Color = Color::WHITE;
const BTN_TXT_CLR_HOV: Color = Color::BLACK;
const BTN_TXT_HSC: &str = "High555coré";
const BTN_TXT_MGN: f32 = 10.0; // PX
const BTN_TXT_FSCR: &str = "Toggle Fullscreen";
const BTN_TXT_MULTIPLAYER: &str = "Multipolar Player";
const BTN_TXT_SINGLEPLAYER: &str = "555inglé Player";
const BTN_TXT_SZE: f32 = 30.0; // Font Size
const BTN_SZE: f32 = 50.0; // Px
const MNU_CLR: Color = Color::rgb(0.15, 0.15, 0.15);
const MNU_PAD: f32 = 5.0; // Px
const TITLE_CLR: Color = Color::PINK;
const TITLE_EMJ: &str = "🐍";
const TITLE_SZE: f32 = 70.0; // Font Size
const TITLE_TXT: &str = "AGENTIFA 555NAKÉ ";
const UI_FILL: f32 = 100.0; //Percent

#[cfg(not(target_arch = "wasm32"))]
const BTN_TXT_QUIT: &str = "Quit";

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MenuEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Menu)
                    .with_system(input_keyboard.after(InputState::Keyboard))
                    .with_system(input_mouse.after(InputState::Mouse))
                    .with_system(navigation)
                    .with_system(update_bg),
            );
    }
}

#[derive(Component)]
struct Background;

#[derive(Clone, Component, Copy, PartialEq)]
enum MenuButton {
    FScr,
    MultiPlayer,
    SinglePlayer,
    HighScore,
    #[cfg(not(target_arch = "wasm32"))]
    Quit,
}

#[derive(Component)]
struct MenuComponent;

#[derive(Component)]
struct MenuState {
    button: MenuButton,
}

impl fmt::Display for MenuButton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            MenuButton::FScr => BTN_TXT_FSCR,
            MenuButton::SinglePlayer => BTN_TXT_SINGLEPLAYER,
            MenuButton::MultiPlayer => BTN_TXT_MULTIPLAYER,
            MenuButton::HighScore => BTN_TXT_HSC,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Quit => BTN_TXT_QUIT,
        };

        write!(f, "{}", str)
    }
}
impl MenuButton {
    fn next(&self) -> Self {
        match *self {
            MenuButton::FScr => MenuButton::MultiPlayer,
            MenuButton::MultiPlayer => MenuButton::SinglePlayer,
            MenuButton::SinglePlayer => MenuButton::HighScore,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::HighScore => MenuButton::Quit,
            #[cfg(target_arch = "wasm32")]
            MenuButton::HighScore => MenuButton::FScr,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Quit => MenuButton::FScr,
        }
    }

    fn prev(&self) -> Self {
        match *self {
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::FScr => MenuButton::Quit,
            #[cfg(target_arch = "wasm32")]
            MenuButton::FScr => MenuButton::HighScore,
            MenuButton::MultiPlayer => MenuButton::FScr,
            MenuButton::SinglePlayer => MenuButton::MultiPlayer,
            MenuButton::HighScore => MenuButton::SinglePlayer,
            #[cfg(not(target_arch = "wasm32"))]
            MenuButton::Quit => MenuButton::HighScore,
        }
    }
}

struct MenuEvent {
    act: Interaction,
    btn: MenuButton,
}

fn border() -> NodeBundle {
    NodeBundle {
        color: BRD_CLR.into(),
        style: Style {
            border: Rect::all(Val::Px(BRD_BRD)),
            size: Size::new(Val::Px(BRD_SZE), Val::Auto),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn button() -> ButtonBundle {
    ButtonBundle {
        color: BTN_CLR.into(),
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(UI_FILL), Val::Px(BTN_SZE)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn button_text(font: Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(BTN_TXT_MGN)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                color: BTN_TXT_CLR,
                font,
                font_size: BTN_TXT_SZE,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<MenuComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn content() -> NodeBundle {
    NodeBundle {
        color: Color::NONE.into(),
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(UI_FILL), Val::Percent(UI_FILL)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::Z),
        ..Default::default()
    }
}

fn input_keyboard(
    mut input: ResMut<Input<KeyCode>>,
    input_state: Res<InputState>,
    menu_state: Query<&MenuState>,
    mut writer: EventWriter<MenuEvent>,
) {
    if !vec![InputState::Keyboard].contains(&input_state) {
        return;
    }

    let mnu = menu_state.iter().next().unwrap();
    if input.pressed(KeyCode::Down) {
        input.release(KeyCode::Down);
        writer.send(MenuEvent {
            act: Interaction::Hovered,
            btn: mnu.button.next(),
        });
    }
    if input.pressed(KeyCode::Up) {
        input.release(KeyCode::Up);
        writer.send(MenuEvent {
            act: Interaction::Hovered,
            btn: mnu.button.prev(),
        });
    }
    if input.just_pressed(KeyCode::Return) {
        input.release(KeyCode::Return);
        writer.send(MenuEvent {
            act: Interaction::Clicked,
            btn: mnu.button,
        });
    }
}

fn input_mouse(
    buttons: Query<(&MenuButton, &GlobalTransform)>,
    input: Res<Input<MouseButton>>,
    input_state: Res<InputState>,
    windows: ResMut<Windows>,
    mut writer: EventWriter<MenuEvent>,
) {
    if !vec![InputState::Mouse].contains(&input_state) {
        return;
    }

    if let Some(cursor) = windows.get_primary().unwrap().cursor_position() {
        for (btn, tf) in buttons.iter() {
            let btn_sze = Vec2::new(BRD_SZE, BTN_SZE);
            let btn_pos = tf.translation.xy();
            let rect = Rect {
                left: btn_pos.x - 0.5 * btn_sze.x,
                right: btn_pos.x + 0.5 * btn_sze.x,
                top: btn_pos.y + 0.5 * btn_sze.y,
                bottom: btn_pos.y - 0.5 * btn_sze.y,
            };

            if cursor.x > rect.left
                && cursor.x < rect.right
                && cursor.y > rect.bottom
                && cursor.y < rect.top
            {
                writer.send(MenuEvent {
                    act: Interaction::Hovered,
                    btn: *btn,
                });

                if input.just_pressed(MouseButton::Left) {
                    writer.send(MenuEvent {
                        act: Interaction::Clicked,
                        btn: *btn,
                    });
                }
            }
        }
    }
}

fn menu() -> NodeBundle {
    NodeBundle {
        color: MNU_CLR.into(),
        style: Style {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            padding: Rect::all(Val::Px(MNU_PAD)),
            size: Size::new(Val::Percent(UI_FILL), Val::Percent(UI_FILL)),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn navigation(
    audio: Res<Audio>,
    mut clear: ResMut<ClearColor>,
    #[cfg(not(target_arch = "wasm32"))] mut exit: EventWriter<AppExit>,
    mut query: Query<(&MenuButton, &Children, &mut UiColor)>,
    mut reader: EventReader<MenuEvent>,
    sounds: Res<AudioAssets>,
    mut state_app: ResMut<State<AppState>>,
    mut state_mnu: Query<&mut MenuState>,
    mut text: Query<&mut Text>,
    mut windows: ResMut<Windows>,
) {
    clear.0 = Color::BLACK;
    let mut mnu = state_mnu.iter_mut().next().unwrap();
    for e in reader.iter() {
        let wnd = windows.get_primary_mut().unwrap();
        match e.act {
            Interaction::Clicked => match e.btn {
                MenuButton::FScr => wnd.set_mode(match wnd.mode() {
                    WindowMode::Windowed => WindowMode::BorderlessFullscreen,
                    _ => WindowMode::Windowed,
                }),
                MenuButton::MultiPlayer => state_app.set(AppState::MultiPlayer).unwrap(),
                MenuButton::SinglePlayer => state_app.set(AppState::SinglePlayer).unwrap(),
                MenuButton::HighScore => state_app.set(AppState::Gameover).unwrap(),
                #[cfg(not(target_arch = "wasm32"))]
                MenuButton::Quit => exit.send(AppExit),
            },
            Interaction::Hovered if e.btn != mnu.button => {
                let click = match random::<u8>() % 3 {
                    0 => sounds.menu_click_1.clone(),
                    1 => sounds.menu_click_2.clone(),
                    _ => sounds.menu_click_3.clone(),
                };

                audio.play(click);
                mnu.button = e.btn;
            }
            _ => {}
        }
    }

    for (btn, chd, mut clr) in query.iter_mut() {
        let mut txt = text.get_mut(chd[0]).unwrap();
        if *btn == mnu.button {
            *clr = BTN_CLR_HOV.into();
            txt.sections[0].style.color = BTN_TXT_CLR_HOV;
        } else {
            *clr = BTN_CLR.into();
            txt.sections[0].style.color = BTN_TXT_CLR;
        }
    }
}

fn root() -> NodeBundle {
    NodeBundle {
        color: Color::NONE.into(),
        style: Style {
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(UI_FILL), Val::Percent(UI_FILL)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::Z),
        ..Default::default()
    }
}

fn setup(
    audio: Res<Audio>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
    images: Res<ImageAssets>,
    sounds: Res<AudioAssets>,
) {
    let spawn_button = |cb: &mut ChildBuilder, btn: MenuButton| {
        cb.spawn_bundle(button())
            .with_children(|parent| {
                parent.spawn_bundle(button_text(
                    fonts.regular.clone(),
                    format!("{}", btn).as_str(),
                ));
            })
            .insert(btn);
    };

    // play music
    audio.stop();
    audio.set_playback_rate(1.);
    audio.play_looped_with_intro(sounds.menu_music_intro.clone(), sounds.menu_music.clone());

    // spawn cameras
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MenuComponent);

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(MenuComponent);

    // spawn background
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BG_CLR.into(),
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture: images.bg_menu.clone(),
            ..Default::default()
        })
        .insert(Background)
        .insert(MenuComponent);

    // spawn ui
    commands
        .spawn_bundle(root())
        .with_children(|parent| {
            // spawn title
            parent.spawn_bundle(title()).with_children(|parent| {
                parent.spawn_bundle(title_text(fonts.bold.clone()));
                parent.spawn_bundle(title_emoji(fonts.emoji.clone()));
            });

            // spawn menu
            parent.spawn_bundle(content()).with_children(|parent| {
                parent.spawn_bundle(border()).with_children(|parent| {
                    parent.spawn_bundle(menu()).with_children(|parent| {
                        spawn_button(parent, MenuButton::FScr);
                        spawn_button(parent, MenuButton::MultiPlayer);
                        spawn_button(parent, MenuButton::SinglePlayer);
                        spawn_button(parent, MenuButton::HighScore);

                        #[cfg(not(target_arch = "wasm32"))]
                        spawn_button(parent, MenuButton::Quit);
                    });
                });
            });
        })
        .insert(MenuComponent);

    // spawn menu state
    commands.spawn().insert(MenuComponent).insert(MenuState {
        button: MenuButton::SinglePlayer,
    });
}

fn title() -> NodeBundle {
    NodeBundle {
        color: Color::NONE.into(),
        style: Style {
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            margin: Rect::all(Val::Px(MNU_PAD)),
            size: Size::new(Val::Percent(UI_FILL), Val::Percent(UI_FILL)),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::Z),
        ..Default::default()
    }
}

fn title_emoji(font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            TITLE_EMJ,
            TextStyle {
                color: TITLE_CLR,
                font,
                font_size: TITLE_SZE,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn title_text(font: Handle<Font>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            TITLE_TXT,
            TextStyle {
                color: TITLE_CLR,
                font,
                font_size: TITLE_SZE,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn update_bg(mut query: Query<&mut Transform, With<Background>>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    for mut tf in query.iter_mut() {
        tf.scale = Vec2::splat(wnd_sze).extend(tf.scale.z);
    }
}
