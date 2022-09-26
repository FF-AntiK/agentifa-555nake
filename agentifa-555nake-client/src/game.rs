use std::collections::HashMap;

use agentifa_555nake_protocol::protocol::{
    DirCmd, Direction, Food, Head, Position, Protocol, QuitCmd, Score, Segment, StartCmd, GRID_SIZE,
};

use bevy::{
    core_pipeline::clear_color::ClearColor,
    input::Input,
    math::{Quat, Vec2, Vec3, Vec4},
    prelude::{
        Added, App, BuildChildren, Camera2dBundle, ChangeTrackers, Changed, Color, Commands,
        Component, DespawnRecursiveExt, Entity, EventReader, KeyCode, MouseButton, NodeBundle, Or,
        ParallelSystemDescriptorCoercion, Plugin, Query, Res, ResMut, State, SystemSet, TextBundle,
        Timer, Transform, UiCameraConfig, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite},
    text::{Text, TextStyle},
    time::Time,
    ui::{AlignItems, JustifyContent, Size, Style, UiRect, Val},
    window::Windows,
};
use bevy_kira_audio::{Audio, AudioControl};
use naia_bevy_client::{events::MessageEvent, shared::DefaultChannels, Client};
use rand::prelude::random;

use crate::{AppState, ImageAssets, InputState, Player, SpriteSheetAssets};
use crate::{AudioAssets, FontAssets};

const AUDIO_RATE_FAKTOR: f64 = 0.01;
const BG_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const BTN_COLOR: Color = Color::rgba(1., 1., 1., 0.5);
const BTN_DIR_IDX: usize = 1;
const BTN_ESC_IDX: usize = 0;
const FOOD_ANIM_CNT: usize = 8;
const HEAD_ANIM_CNT: usize = 4;
const HEAD_COLOR: Color = Color::WHITE;
const HEAD_COLOR_L: f32 = 0.5;
const HEAD_COLOR_S: f32 = 1.;
const HEAD_COLOR_SPEED: f32 = 0.01;
const INVINCIBLE_DUR: f32 = 0.25;
const SCOREBAR_COLOR: Color = Color::GRAY;
const SCORETEXT_COLOR: Color = Color::YELLOW;
const SEGMENT_ANIM_CNT: usize = 6;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(assign_message)
                    .with_system(input_keyboard.after(InputState::Keyboard))
                    .with_system(input_mouse.after(InputState::Mouse))
                    .with_system(quit_command)
                    .with_system(update_audio)
                    .with_system(update_background)
                    .with_system(update_buttons.after(InputState::Mouse))
                    .with_system(update_dimensions)
                    .with_system(update_foods)
                    .with_system(update_heads)
                    .with_system(update_head_color)
                    .with_system(update_head_dir)
                    .with_system(update_positions)
                    .with_system(update_scores)
                    .with_system(update_segments)
                    .with_system(update_sheets)
                    .with_system(update_scorebar)
                    .with_system(update_scorebar_container)
                    .with_system(update_scorecoin)
                    .with_system(update_scoretext),
            )
            .insert_resource(Dimensions::default())
            .insert_resource(Scores {
                list: HashMap::new(),
            });
    }
}

#[derive(Component)]
struct Animation {
    count: usize,
}

#[derive(Component)]
struct Background;

#[derive(Default)]
struct Dimensions {
    blk: f32,
    wnd_h: f32,
    wnd_w: f32,
    wnd_max: f32,
    wnd_min: f32,
}

#[derive(Component)]
enum Button {
    Escape,
    Direction(Direction),
}

#[derive(Component)]
struct Coin;

#[derive(Component)]
struct HeadLocal {
    color_dst: Color,
    color_src: Color,
    invincible: bool,
    timer: Timer,
}

#[derive(Component)]
struct Local;

#[derive(Component)]
struct Own;

#[derive(Component)]
struct Remote;

struct Scores {
    list: HashMap<Entity, usize>,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct ScoreBar;

#[derive(Component)]
struct ScoreBarContainer;

fn assign_message(
    client: Client<Protocol, DefaultChannels>,
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, DefaultChannels>>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(_, Protocol::AssignMsg(msg)) = event {
            commands
                .entity(msg.entity.get(&client).unwrap())
                .insert(Own);
        }
    }
}

fn cleanup(
    mut client: Client<Protocol, DefaultChannels>,
    mut commands: Commands,
    local: Query<Entity, With<Local>>,
    remote: Query<Entity, With<Remote>>,
) {
    client.send_message(DefaultChannels::UnorderedReliable, &QuitCmd::new());
    for entity in local.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in remote.iter() {
        commands
            .entity(entity)
            .remove::<Animation>()
            .remove::<HeadLocal>()
            .remove::<Remote>()
            .remove_bundle::<SpriteSheetBundle>();
    }
}

fn input_keyboard(
    mut app_state: ResMut<State<AppState>>,
    mut client: Client<Protocol, DefaultChannels>,
    mut input: ResMut<Input<KeyCode>>,
    input_state: Res<InputState>,
) {
    if !vec![InputState::Keyboard].contains(&input_state) {
        return;
    }

    if input.pressed(KeyCode::Escape) {
        input.release(KeyCode::Escape);
        app_state.set(AppState::Menu).unwrap();
        return;
    }

    if input.pressed(KeyCode::Down) {
        input.release(KeyCode::Down);
        client.send_message(
            DefaultChannels::UnorderedReliable,
            &DirCmd::new(Direction::Down),
        );
    }
    if input.pressed(KeyCode::Left) {
        input.release(KeyCode::Left);
        client.send_message(
            DefaultChannels::UnorderedReliable,
            &DirCmd::new(Direction::Left),
        );
    }
    if input.pressed(KeyCode::Right) {
        input.release(KeyCode::Right);
        client.send_message(
            DefaultChannels::UnorderedReliable,
            &DirCmd::new(Direction::Right),
        );
    }
    if input.pressed(KeyCode::Up) {
        input.release(KeyCode::Up);
        client.send_message(
            DefaultChannels::UnorderedReliable,
            &DirCmd::new(Direction::Up),
        );
    }
}

fn input_mouse(
    mut app_state: ResMut<State<AppState>>,
    buttons: Query<(&Button, &Transform)>,
    mut client: Client<Protocol, DefaultChannels>,
    input: Res<Input<MouseButton>>,
    input_state: Res<InputState>,
    windows: Res<Windows>,
) {
    if !vec![InputState::Mouse].contains(&input_state) {
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        let wnd = windows.get_primary().unwrap();
        if let Some(mut cursor) = wnd.cursor_position() {
            cursor -= 0.5 * Vec2::new(wnd.width(), wnd.height());
            let contains = |p: Vec2, a: UiRect<f32>| {
                p.x > a.left && p.x < a.right && p.y > a.bottom && p.y < a.top
            };

            for (btn, tf) in buttons.iter() {
                let offs = 0.5 * tf.scale;
                let rect = UiRect {
                    bottom: tf.translation.y - offs.y,
                    left: tf.translation.x - offs.x,
                    right: tf.translation.x + offs.x,
                    top: tf.translation.y + offs.y,
                };

                if contains(cursor, rect) {
                    match *btn {
                        Button::Escape => app_state.set(AppState::Menu).unwrap(),
                        Button::Direction(dir) => client
                            .send_message(DefaultChannels::UnorderedReliable, &DirCmd::new(dir)),
                    }
                }
            }
        }
    }
}

fn quit_command(
    mut app_state: ResMut<State<AppState>>,
    mut event_reader: EventReader<MessageEvent<Protocol, DefaultChannels>>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(_, Protocol::QuitCmd(_)) = event {
            app_state.set(AppState::Gameover).unwrap();
        }
    }
}

fn setup(
    audio: Res<Audio>,
    mut clear: ResMut<ClearColor>,
    mut client: Client<Protocol, DefaultChannels>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
    images: Res<ImageAssets>,
    player: Res<Player>,
    sheets: Res<SpriteSheetAssets>,
    sounds: Res<AudioAssets>,
) {
    let spawn_dirbtn = |cmd: &mut Commands, dir: Direction| {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: BTN_COLOR,
                custom_size: Some(Vec2::ONE),
                index: BTN_DIR_IDX,
                ..Default::default()
            },
            texture_atlas: sheets.keys.clone(),
            transform: Transform::from_translation(2. * Vec3::Z),
            ..Default::default()
        })
        .insert(Button::Direction(dir))
        .insert(Local);
    };

    clear.0 = Color::DARK_GRAY;

    // play music
    audio.stop();
    audio.set_playback_rate(1.);
    audio.play(sounds.game_music.clone()).looped();

    // spawn camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(Local)
        .insert(UiCameraConfig { show_ui: true });

    // spawn background
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BG_COLOR,
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture: images.bg_game.clone(),
            ..Default::default()
        })
        .insert(Background)
        .insert(Local);

    // spawn scores
    commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            // spawn score container
            p.spawn_bundle(NodeBundle {
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|p| {
                // spawn score bar
                p.spawn_bundle(NodeBundle {
                    color: SCOREBAR_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|p| {
                    // spawn coin image
                    p.spawn_bundle(NodeBundle {
                        color: Color::WHITE.into(),
                        image: images.diamond.clone().into(),
                        transform: Transform::from_scale(Vec3::new(0.9, 0.9, 1.)),
                        ..Default::default()
                    })
                    .insert(Coin);

                    // spawn score text
                    p.spawn_bundle(TextBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                color: SCORETEXT_COLOR,
                                font: fonts.regular.clone(),
                                font_size: 0.,
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(ScoreText);
                })
                .insert(ScoreBar);
            })
            .insert(ScoreBarContainer);
        })
        .insert(Local);

    // spawn buttons
    spawn_dirbtn(&mut commands, Direction::Down);
    spawn_dirbtn(&mut commands, Direction::Left);
    spawn_dirbtn(&mut commands, Direction::Right);
    spawn_dirbtn(&mut commands, Direction::Up);
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: BTN_COLOR,
                custom_size: Some(Vec2::ONE),
                index: BTN_ESC_IDX,
                ..Default::default()
            },
            texture_atlas: sheets.keys.clone(),
            transform: Transform::from_translation(2. * Vec3::Z),
            ..Default::default()
        })
        .insert(Button::Escape)
        .insert(Local);

    // spawn player
    client.send_message(
        DefaultChannels::UnorderedReliable,
        &StartCmd::new(player.name.clone()),
    );
}

fn update_audio(audio: Res<Audio>, query: Query<&Score, (Changed<Score>, With<Own>)>) {
    for score in query.iter() {
        audio.set_playback_rate(1. + AUDIO_RATE_FAKTOR * *score.level as f64);
    }
}

fn update_background(
    mut backgrounds: Query<&mut Transform, With<Background>>,
    dimensions: Res<Dimensions>,
) {
    if dimensions.is_changed() {
        let mut tf = backgrounds.single_mut();
        tf.translation.y = -0.5 * dimensions.blk;
        tf.scale = Vec2::splat(dimensions.blk * (GRID_SIZE as f32)).extend(tf.scale.z);
    }
}

fn update_dimensions(mut dimensions: ResMut<Dimensions>, windows: Res<Windows>) {
    if windows.is_changed() {
        let wnd = windows.get_primary().unwrap();
        dimensions.wnd_h = wnd.height();
        dimensions.wnd_w = wnd.width();
        dimensions.wnd_max = f32::max(dimensions.wnd_h, dimensions.wnd_w);
        dimensions.wnd_min = f32::min(dimensions.wnd_h, dimensions.wnd_w);
        dimensions.blk = dimensions.wnd_min / (GRID_SIZE + 1) as f32;
    }
}

fn update_buttons(
    mut buttons: Query<(&Button, &mut TextureAtlasSprite, &mut Transform)>,
    dimensions: Res<Dimensions>,
    input_state: Res<InputState>,
) {
    let wnd_sze = Vec2::new(dimensions.wnd_w, dimensions.wnd_h);
    let offs = 0.5 * wnd_sze;
    for (btn, mut tex, mut tf) in buttons.iter_mut() {
        if dimensions.is_changed() {
            tf.scale = Vec2::splat(2. * dimensions.blk).extend(tf.scale.z);
            match *btn {
                Button::Escape => {
                    tf.translation = (1.5 * dimensions.blk - offs).extend(tf.translation.z);
                }
                Button::Direction(dir) => {
                    let offs = (Vec2::Y - Vec2::X) * offs;
                    let pos = match dir {
                        Direction::Down => Vec2::new(-3.5, 1.5),
                        Direction::Left => Vec2::new(-5.5, 1.5),
                        Direction::Right => Vec2::new(-1.5, 1.5),
                        Direction::Up => Vec2::new(-3.5, 3.5),
                    };

                    tex.flip_x = dir.flip_x();
                    tex.flip_y = dir.flip_y();
                    tf.rotation = Quat::from_rotation_z(dir.angle());
                    tf.translation = (pos * dimensions.blk - offs).extend(tf.translation.z);
                }
            };
        }

        if input_state.is_changed() {
            tex.color = match *input_state {
                InputState::Keyboard => Color::NONE,
                InputState::Mouse => BTN_COLOR,
            };
        }
    }
}

fn update_foods(
    mut commands: Commands,
    query: Query<Entity, (With<Food>, Without<Remote>)>,
    sheets: Res<SpriteSheetAssets>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(Animation {
                count: FOOD_ANIM_CNT,
            })
            .insert(Remote)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: sheets.food.clone(),
                transform: Transform::from_translation(Vec3::Z),
                ..Default::default()
            });
    }
}

fn update_heads(
    mut commands: Commands,
    mut query: Query<Entity, (With<Head>, Without<Remote>)>,
    sheets: Res<SpriteSheetAssets>,
) {
    for entity in query.iter_mut() {
        commands
            .entity(entity)
            .insert(Animation {
                count: HEAD_ANIM_CNT,
            })
            .insert(Remote)
            .insert(HeadLocal {
                color_dst: HEAD_COLOR,
                color_src: HEAD_COLOR,
                invincible: true,
                timer: Timer::from_seconds(INVINCIBLE_DUR, true),
            })
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: HEAD_COLOR,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: sheets.pimmler.clone(),
                transform: Transform::from_translation(Vec3::Z),
                ..Default::default()
            });
    }
}

fn update_head_color(
    mut query: Query<(&mut HeadLocal, &Score, &mut TextureAtlasSprite), With<Own>>,
    time: Res<Time>,
) {
    for (mut head, score, mut sprite) in query.iter_mut() {
        let mut src: Vec4 = head.color_src.into();
        let dst: Vec4 = head.color_dst.into();
        let dir = Vec4::normalize(dst - src);
        if dir.is_finite() {
            src += HEAD_COLOR_SPEED * dir;
            head.color_src = src.into();
        };

        if dst.distance(src) < HEAD_COLOR_SPEED {
            head.color_dst = Color::hsl(random::<f32>() * 360.0, HEAD_COLOR_S, HEAD_COLOR_L);
        }

        if head.invincible && *score.level > 0 {
            head.invincible = false;
        } else if *score.level == 0 && head.timer.tick(time.delta()).just_finished() {
            head.invincible = !head.invincible;
        }

        if head.invincible {
            sprite.color = Color::WHITE;
        } else {
            sprite.color = src.into();
        }
    }
}

fn update_head_dir(
    mut query: Query<
        (&Head, &mut TextureAtlasSprite, &mut Transform),
        Or<(Added<Remote>, Changed<Head>)>,
    >,
) {
    for (head, mut sprite, mut tf) in query.iter_mut() {
        sprite.flip_x = head.dir.flip_x();
        sprite.flip_y = head.dir.flip_y();
        tf.rotation = Quat::from_rotation_z(head.dir.angle());
    }
}

fn update_positions(
    dimensions: Res<Dimensions>,
    mut positions: Query<(ChangeTrackers<Position>, &Position, &mut Transform)>,
) {
    for (tracker, pos, mut tf) in positions.iter_mut() {
        if dimensions.is_changed() || tracker.is_changed() {
            let grid = Vec2::new(
                dimensions.blk * (GRID_SIZE as f32),
                dimensions.blk * ((GRID_SIZE + 1) as f32),
            );

            let offs = 0.5 * (grid - dimensions.blk);
            tf.translation = (Vec2::new(*pos.x as f32, *pos.y as f32) * dimensions.blk - offs)
                .extend(tf.translation.z);
        }

        if dimensions.is_changed() {
            tf.scale = Vec3::new(dimensions.blk, dimensions.blk, tf.scale.z);
        }
    }
}

fn update_scorebar(mut bars: Query<&mut Style, With<ScoreBar>>, dimensions: Res<Dimensions>) {
    if dimensions.is_changed() {
        let mut style = bars.iter_mut().next().unwrap();
        style.size = Size::new(
            Val::Px(dimensions.blk * GRID_SIZE as f32),
            Val::Px(dimensions.blk),
        );
    }
}

fn update_scorebar_container(
    mut containers: Query<&mut Style, With<ScoreBarContainer>>,
    dimensions: Res<Dimensions>,
) {
    if dimensions.is_changed() {
        let mut style = containers.iter_mut().next().unwrap();
        style.size = Size::new(
            Val::Px(dimensions.blk * GRID_SIZE as f32),
            Val::Px(dimensions.blk + 0.5 * (dimensions.wnd_h - dimensions.wnd_min)),
        );
    }
}

fn update_scorecoin(mut coins: Query<&mut Style, With<Coin>>, dimensions: Res<Dimensions>) {
    if dimensions.is_changed() {
        let mut style = coins.iter_mut().next().unwrap();
        style.size = Size::new(Val::Px(dimensions.blk), Val::Px(dimensions.blk));
    }
}

fn update_scores(
    audio: Res<Audio>,
    query: Query<(ChangeTrackers<Score>, Entity, &Score)>,
    mut scores: ResMut<Scores>,
    sounds: Res<AudioAssets>,
) {
    scores.list.retain(|k, _| query.get(*k).is_ok());
    for (tracker, entity, score) in query.iter() {
        if tracker.is_changed() {
            //TODO: why is this neccessary
            if let Some(s) = scores.list.get_mut(&entity) {
                if *s != *score.level {
                    audio.play(sounds.game_eat.clone());
                    *s = *score.level;
                }
            } else {
                scores.list.insert(entity, *score.level);
            }
        }
    }
}

fn update_scoretext(
    dimensions: Res<Dimensions>,
    player: Res<Player>,
    scores: Query<&Score, (Or<(Added<Own>, Changed<Score>)>, With<Own>)>,
    mut texts: Query<&mut Text, With<ScoreText>>,
) {
    let mut txt = texts.iter_mut().next().unwrap();
    if dimensions.is_changed() {
        txt.sections[0].style.font_size = dimensions.blk;
    }

    for score in scores.iter() {
        txt.sections[0].value = format!("X{} {}", *score.level, player.name);
    }
}

fn update_segments(
    mut commands: Commands,
    query: Query<(Entity, &Segment), Without<Remote>>,
    sheets: Res<SpriteSheetAssets>,
) {
    for (entity, segment) in query.iter() {
        if !*segment.synced {
            continue;
        }

        commands
            .entity(entity)
            .insert(Animation {
                count: SEGMENT_ANIM_CNT,
            })
            .insert(Remote)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::ONE),
                    ..Default::default()
                },
                texture_atlas: sheets.diamond.clone(),
                transform: Transform::from_translation(Vec3::Z),
                ..Default::default()
            });
    }
}

fn update_sheets(mut query: Query<(&Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    let cnt = 10.0 * time.seconds_since_startup();
    for (a, mut sheet) in query.iter_mut() {
        sheet.index = cnt as usize % a.count;
    }
}
