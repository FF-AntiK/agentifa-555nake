use agentifa_555nake_protocol::protocol::{HighScore, HighScoreRank};
use bevy::{
    core_pipeline::clear_color::ClearColor,
    input::Input,
    math::{Vec2, Vec3},
    prelude::{
        App, BuildChildren, Camera2dBundle, Color, Commands, Component, DespawnRecursiveExt,
        Entity, KeyCode, MouseButton, NodeBundle, ParallelSystemDescriptorCoercion, Plugin, Query,
        Res, ResMut, State, SystemSet, TextBundle, Transform, UiCameraConfig, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite},
    text::{
        HorizontalAlign, Text, Text2dBundle, TextAlignment, TextSection, TextStyle, VerticalAlign,
    },
    time::Time,
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
    window::Windows,
};
use bevy_kira_audio::{Audio, AudioControl};

use crate::{AppState, ImageAssets, InputState, SpriteSheetAssets};
use crate::{AudioAssets, FontAssets};

const BTN_COLOR: Color = Color::rgba(1., 1., 1., 0.5);
const BTN_ESC_IDX: usize = 0;
const ENTRY_SIZE: f32 = 30.;
const GRID_SIZE: u8 = 10;
const NAME_COLOR: Color = Color::CYAN;
const RANK_COLOR: Color = Color::PINK;
const SCORE_COLOR: Color = Color::YELLOW;
const SCROLL_SPEED: f32 = 55.5;
const TITLE_SIZE: f32 = 75.;
const TITLE_TEXT: &str = "HIGH555CORÉ";
const TITLE_COLOR: Color = Color::YELLOW;

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Gameover).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Gameover).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Gameover)
                    .with_system(input_keyboard.after(InputState::Keyboard))
                    .with_system(input_mouse.after(InputState::Mouse))
                    .with_system(insert_entries)
                    .with_system(update_background)
                    .with_system(update_buttons)
                    .with_system(update_credits)
                    .with_system(update_entries)
                    .with_system(update_scrolling),
            )
            .insert_resource(CreditCount(0))
            .insert_resource(ScrollPosition(0.));
    }
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Button;

#[derive(Component)]
struct Credit {
    position: usize,
}

struct CreditCount(usize);

#[derive(Component)]
struct GameOverComponent;

struct ScrollPosition(f32);

fn cleanup(
    mut commands: Commands,
    entries: Query<Entity, (With<HighScoreRank>, With<Text>)>,
    query: Query<Entity, With<GameOverComponent>>,
) {
    for entity in entries.iter() {
        commands.entity(entity).remove_bundle::<Text2dBundle>();
    }

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn input_keyboard(
    mut app_state: ResMut<State<AppState>>,
    mut input: ResMut<Input<KeyCode>>,
    input_state: Res<InputState>,
) {
    if !vec![InputState::Keyboard].contains(&input_state) {
        return;
    }

    if input.pressed(KeyCode::Escape) {
        input.release(KeyCode::Escape);
        app_state.set(AppState::Menu).unwrap();
    }
}

fn input_mouse(
    mut app_state: ResMut<State<AppState>>,
    buttons: Query<&Transform, With<Button>>,
    input: Res<Input<MouseButton>>,
    input_state: Res<InputState>,
    windows: Res<Windows>,
) {
    if !vec![InputState::Mouse].contains(&input_state) || !input.just_pressed(MouseButton::Left) {
        return;
    }

    let wnd = windows.get_primary().unwrap();
    if let Some(mut cursor) = wnd.cursor_position() {
        cursor -= 0.5 * Vec2::new(wnd.width(), wnd.height());
        let contains = |p: Vec2, a: UiRect<f32>| {
            p.x > a.left && p.x < a.right && p.y > a.bottom && p.y < a.top
        };

        let tf = buttons.get_single().unwrap();
        let offs = 0.5 * tf.scale;
        let rect = UiRect {
            bottom: tf.translation.y - offs.y,
            left: tf.translation.x - offs.x,
            right: tf.translation.x + offs.x,
            top: tf.translation.y + offs.y,
        };

        if contains(cursor, rect) {
            app_state.set(AppState::Menu).unwrap();
        }
    }
}

fn insert_entries(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    scroll_pos: Res<ScrollPosition>,
    query_invisible: Query<(Entity, &HighScoreRank), Without<Text>>,
    query_visible: Query<(Entity, &HighScoreRank), With<Text>>,
    windows: Res<Windows>,
) {
    let section = |c: Color| -> TextSection {
        TextSection {
            value: String::new(),
            style: TextStyle {
                font: fonts.regular.clone(),
                font_size: ENTRY_SIZE,
                color: c,
            },
        }
    };

    let height = windows.get_primary().unwrap().height();
    let visible = |p: f32| -> bool { p >= scroll_pos.0 - height && p <= scroll_pos.0 };
    for (entity, HighScoreRank { position, .. }) in query_invisible.iter() {
        if visible(ENTRY_SIZE * *position.clone() as f32) {
            commands.entity(entity).insert_bundle(Text2dBundle {
                text: Text {
                    sections: vec![
                        section(RANK_COLOR),
                        section(NAME_COLOR),
                        section(SCORE_COLOR),
                    ],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                },
                transform: Transform::from_translation(Vec3::Z),
                ..Default::default()
            });
        }
    }

    for (entity, HighScoreRank { position, .. }) in query_visible.iter() {
        if !visible(ENTRY_SIZE * *position.clone() as f32) {
            commands.entity(entity).remove_bundle::<Text2dBundle>();
        }
    }
}

fn setup(
    audio: Res<Audio>,
    mut clear: ResMut<ClearColor>,
    mut commands: Commands,
    mut credit_count: ResMut<CreditCount>,
    fonts: Res<FontAssets>,
    images: Res<ImageAssets>,
    sheets: Res<SpriteSheetAssets>,
    mut position: ResMut<ScrollPosition>,
    sounds: Res<AudioAssets>,
) {
    credit_count.0 = 0;
    let mut credit = |cmd: &mut Commands, text: &str| {
        cmd.spawn_bundle(Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: fonts.regular.clone(),
                    font_size: ENTRY_SIZE,
                    color: NAME_COLOR,
                },
            )
            .with_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            }),
            transform: Transform::from_translation(Vec3::Z),
            ..Default::default()
        })
        .insert(Credit {
            position: credit_count.0,
        })
        .insert(GameOverComponent);
        credit_count.0 += 1;
    };

    clear.0 = Color::BLACK;
    audio.stop();
    audio.set_playback_rate(1.);
    audio.play(sounds.game_over.clone()).looped();

    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameOverComponent)
        .insert(UiCameraConfig { show_ui: true });

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: BTN_COLOR,
                custom_size: Some(Vec2::ONE),
                index: BTN_ESC_IDX,
                ..Default::default()
            },
            texture_atlas: sheets.keys.clone(),
            transform: Transform::from_translation(Vec3::Z),
            ..Default::default()
        })
        .insert(GameOverComponent)
        .insert(Button);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.1),
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture: images.game_over.clone(),
            ..Default::default()
        })
        .insert(Background)
        .insert(GameOverComponent);

    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_bundle(NodeBundle {
                color: Color::BLACK.into(),
                style: Style {
                    justify_content: JustifyContent::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(TITLE_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|p| {
                p.spawn_bundle(TextBundle {
                    text: Text::from_section(
                        TITLE_TEXT,
                        TextStyle {
                            color: TITLE_COLOR,
                            font: fonts.bold.clone(),
                            font_size: TITLE_SIZE,
                        },
                    ),
                    ..Default::default()
                });
            });
        })
        .insert(GameOverComponent);

    credit(&mut commands, "Credits");
    credit(&mut commands, "-------");
    credit(&mut commands, "");
    credit(&mut commands, "Code:");
    credit(&mut commands, "Agent FF AntiK");
    credit(&mut commands, "");
    credit(&mut commands, "Music:");
    credit(&mut commands, "Agent FF AntiK");
    credit(&mut commands, "Roman B");
    credit(&mut commands, "");
    credit(&mut commands, "SFX:");
    credit(&mut commands, "Agent FF AntiK");
    credit(&mut commands, "");
    credit(&mut commands, "Sprites:");
    credit(&mut commands, "7onas");
    credit(&mut commands, "Agent FF AntiK");
    credit(&mut commands, "");
    credit(&mut commands, "");
    credit(&mut commands, "Special Thanks");
    credit(&mut commands, "--------------");
    credit(&mut commands, "");
    credit(&mut commands, "Connor Carpenter");
    credit(&mut commands, "for the incredible naia support");
    credit(&mut commands, "");
    credit(&mut commands, "Agent 00 Bielefeld");
    credit(&mut commands, "aka One Man Mafia");
    credit(&mut commands, "alias Der Pimmler");
    credit(&mut commands, "");
    credit(&mut commands, "AGENTIFA 555 OINK NASÉ");
    credit(&mut commands, "");
    credit(&mut commands, "");
    credit(&mut commands, "");
    position.0 = -1. * ENTRY_SIZE * credit_count.0 as f32;
}

fn update_background(mut query: Query<&mut Transform, With<Background>>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    for mut tf in query.iter_mut() {
        tf.scale = Vec2::splat(wnd_sze).extend(tf.scale.z);
    }
}

fn update_buttons(
    mut buttons: Query<(&mut TextureAtlasSprite, &mut Transform), With<Button>>,
    input_state: Res<InputState>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = Vec2::new(wnd.width(), wnd.height());
    let blk = wnd_sze.min_element() / (GRID_SIZE + 1) as f32;
    let offs = 0.5 * wnd_sze;
    let (mut tex, mut tf) = buttons.single_mut();

    tex.color = match *input_state {
        InputState::Keyboard => Color::NONE,
        InputState::Mouse => BTN_COLOR,
    };

    tf.scale = Vec2::splat(2. * blk).extend(tf.scale.z);
    tf.translation = (1.5 * blk - offs).extend(tf.translation.z);
}

fn update_credits(
    credit_count: Res<CreditCount>,
    mut query: Query<(&Credit, &mut Transform)>,
    scroll_pos: Res<ScrollPosition>,
    windows: Res<Windows>,
) {
    let y_offs = 0.5 * windows.get_primary().unwrap().height() - ENTRY_SIZE * credit_count.0 as f32;
    for (Credit { position }, mut tf) in query.iter_mut() {
        tf.translation.y = scroll_pos.0 - y_offs - ENTRY_SIZE * *position as f32;
    }
}

fn update_entries(
    mut query: Query<(&HighScore, &HighScoreRank, &mut Text, &mut Transform)>,
    scroll_pos: Res<ScrollPosition>,
    windows: Res<Windows>,
) {
    let y_offs = 0.5 * windows.get_primary().unwrap().height();
    for (HighScore { name, score }, HighScoreRank { position, rank }, mut txt, mut tf) in
        query.iter_mut()
    {
        let n = &*name.clone();
        let p = *position.clone() as f32;
        let r = rank.wrapping_add(1);
        let s = *score.clone();
        txt.sections[0].value = format!("{: >4}. ", r);
        txt.sections[1].value = format!("{: <30} ", n);
        txt.sections[2].value = format!("{: >4}", s);
        tf.translation.y = scroll_pos.0 - y_offs - ENTRY_SIZE * p;
    }
}

fn update_scrolling(mut scroll_pos: ResMut<ScrollPosition>, time: Res<Time>) {
    scroll_pos.0 += SCROLL_SPEED * time.delta_seconds();
}
