use std::{f32::consts::PI, fmt};

use agentifa_555nake_protocol::protocol::{HighScore, Protocol};

use bevy::{
    core_pipeline::clear_color::ClearColor,
    input::Input,
    math::{Quat, Vec2, Vec3, Vec4},
    prelude::{
        App, BuildChildren, Camera2dBundle, Color, Commands, Component, DespawnRecursiveExt,
        Entity, KeyCode, MouseButton, NodeBundle, ParallelSystemDescriptorCoercion, Plugin, Query,
        Res, ResMut, State, SystemSet, TextBundle, Transform, UiCameraConfig, With, Without,
    },
    sprite::{Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite},
    text::{Text, TextStyle},
    time::{Time, Timer},
    ui::{AlignItems, JustifyContent, Size, Style, UiRect, Val},
    window::Windows,
};
use bevy_kira_audio::{Audio, AudioControl};
use naia_bevy_client::Client;
use naia_shared::DefaultChannels;
use rand::{
    distributions::Standard,
    prelude::{random, Distribution},
};

use crate::{AppState, ImageAssets, InputState, Player, SpriteSheetAssets};
use crate::{AudioAssets, FontAssets};

const BG_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const BTN_COLOR: Color = Color::rgba(1., 1., 1., 0.5);
const BTN_DIR_IDX: usize = 1;
const BTN_ESC_IDX: usize = 0;
const FOOD_ANIM_CNT: usize = 8;
const FOOD_SPAWN_DUR: f32 = 3.0;
const GRID_SIZE: usize = 10;
const HEAD_ANIM_CNT: usize = 4;
const HEAD_COLOR_L: f32 = 0.5;
const HEAD_COLOR_S: f32 = 1.;
const HEAD_COLOR_SPEED: f32 = 0.01;
const HEAD_MOV_DUR: f32 = 0.5;
const SCOREBAR_COLOR: Color = Color::GRAY;
const SCORETEXT_COLOR: Color = Color::YELLOW;
const SEGMENT_ANIM_CNT: usize = 6;
const STARTPOS: Position = Position { x: 5, y: 5 };

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(input_keyboard.after(InputState::Keyboard))
                    .with_system(input_mouse.after(InputState::Mouse))
                    .with_system(update_background)
                    .with_system(update_buttons.after(InputState::Mouse))
                    .with_system(update_foodspawner)
                    .with_system(update_head_color)
                    .with_system(update_head_dir)
                    .with_system(update_head_position)
                    .with_system(update_positions)
                    .with_system(update_sheets)
                    .with_system(update_scorebar)
                    .with_system(update_scorebar_container)
                    .with_system(update_scorecoin)
                    .with_system(update_scoretext),
            );
    }
}

#[derive(Component)]
struct Animation {
    count: usize,
}

#[derive(Component)]
struct AudioTrack {
    rate: f64,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
enum Button {
    Escape,
    Direction(Direction),
}

#[derive(Component)]
struct Coin;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Distribution<Direction> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..3) {
            0 => Direction::Down,
            1 => Direction::Left,
            2 => Direction::Right,
            _ => Direction::Up,
        }
    }
}

impl Direction {
    fn angle(self) -> f32 {
        PI * match self {
            Direction::Left => 0.0,
            Direction::Right => 1.0,
            _ => 0.5,
        }
    }

    fn flip_x(self) -> bool {
        match self {
            Direction::Up => true,
            _ => false,
        }
    }

    fn flip_y(self) -> bool {
        match self {
            Direction::Right => true,
            _ => false,
        }
    }
}

#[derive(Component, Default)]
struct Food;

#[derive(Component)]
struct FoodSpawner {
    timer: Timer,
}

#[derive(Component)]
struct GameComponent;

#[derive(Component)]
struct Head {
    color: Color,
    dir: Direction,
    timer: Timer,
}

#[derive(Clone, Component, Copy, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl Position {
    fn rnd() -> Self {
        Position {
            x: random::<usize>() % GRID_SIZE,
            y: random::<usize>() % GRID_SIZE,
        }
    }
}

#[derive(Component, Default)]
struct Score {
    count: usize,
}

#[derive(Component)]
struct ScoreBar;

#[derive(Component)]
struct ScoreBarContainer;

#[derive(Clone, Component, Copy, Default)]
struct Segment {
    index: usize,
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<GameComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_blocksize(wnd: f32) -> f32 {
    wnd / (GRID_SIZE + 1) as f32
}

fn input_keyboard(
    mut app_state: ResMut<State<AppState>>,
    mut heads: Query<&mut Head>,
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

    for mut head in heads.iter_mut() {
        if input.pressed(KeyCode::Down) {
            input.release(KeyCode::Down);
            head.dir = Direction::Down;
        }
        if input.pressed(KeyCode::Left) {
            input.release(KeyCode::Left);
            head.dir = Direction::Left;
        }
        if input.pressed(KeyCode::Right) {
            input.release(KeyCode::Right);
            head.dir = Direction::Right;
        }
        if input.pressed(KeyCode::Up) {
            input.release(KeyCode::Up);
            head.dir = Direction::Up;
        }
    }
}

fn input_mouse(
    mut app_state: ResMut<State<AppState>>,
    buttons: Query<(&Button, &Transform)>,
    mut heads: Query<&mut Head>,
    input: Res<Input<MouseButton>>,
    input_state: Res<InputState>,
    windows: Res<Windows>,
) {
    if !vec![InputState::Mouse].contains(&input_state) {
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        let mut head = heads.iter_mut().next().unwrap();
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
                        Button::Direction(dir) => head.dir = dir,
                    }
                }
            }
        }
    }
}

fn setup(
    audio: Res<Audio>,
    mut clear: ResMut<ClearColor>,
    mut commands: Commands,
    fonts: Res<FontAssets>,
    images: Res<ImageAssets>,
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
        .insert(GameComponent)
        .insert(Button::Direction(dir));
    };

    clear.0 = Color::DARK_GRAY;

    // play music
    audio.stop();
    audio.set_playback_rate(1.);
    audio.play(sounds.game_music.clone()).looped();
    commands
        .spawn()
        .insert(AudioTrack { rate: 1. })
        .insert(GameComponent);

    // spawn camera
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(GameComponent)
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
        .insert(GameComponent);

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
                    .insert(Score::default());
                })
                .insert(ScoreBar);
            })
            .insert(ScoreBarContainer);
        })
        .insert(GameComponent);

    // spawn head
    let clr: Color = Color::PINK;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: clr,
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture_atlas: sheets.pimmler.clone(),
            transform: Transform::from_translation(Vec3::Z),
            ..Default::default()
        })
        .insert(Animation {
            count: HEAD_ANIM_CNT,
        })
        .insert(GameComponent)
        .insert(Head {
            color: clr,
            dir: rand::random(),
            timer: Timer::from_seconds(HEAD_MOV_DUR, true),
        })
        .insert(STARTPOS)
        .insert(Segment::default());

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
        .insert(GameComponent)
        .insert(Button::Escape);

    // spawn food spawner
    commands
        .spawn()
        .insert(FoodSpawner {
            timer: Timer::from_seconds(FOOD_SPAWN_DUR, false),
        })
        .insert(GameComponent);
}

fn transform_position(blk: f32, pos: Position) -> Vec2 {
    let grid = Vec2::new(blk * (GRID_SIZE as f32), blk * ((GRID_SIZE + 1) as f32));
    let offs = 0.5 * (grid - blk);
    Vec2::new(pos.x as f32, pos.y as f32) * blk - offs
}

fn update_background(
    mut backgrounds: Query<&mut Transform, With<Background>>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let blk = get_blocksize(f32::min(wnd.height(), wnd.width()));
    for mut tf in backgrounds.iter_mut() {
        tf.translation.y = -0.5 * blk;
        tf.scale = Vec2::splat(blk * (GRID_SIZE as f32)).extend(tf.scale.z);
    }
}

fn update_buttons(
    mut buttons: Query<(&Button, &mut TextureAtlasSprite, &mut Transform)>,
    input_state: Res<InputState>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = Vec2::new(wnd.width(), wnd.height());
    let blk = get_blocksize(wnd_sze.min_element());
    let offs = 0.5 * wnd_sze;
    for (btn, mut tex, mut tf) in buttons.iter_mut() {
        tf.scale = Vec2::splat(2. * blk).extend(tf.scale.z);
        tex.color = match *input_state {
            InputState::Keyboard => Color::NONE,
            InputState::Mouse => BTN_COLOR,
        };

        match *btn {
            Button::Escape => {
                tf.translation = (1.5 * blk - offs).extend(tf.translation.z);
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
                tf.translation = (pos * blk - offs).extend(tf.translation.z);
            }
        };
    }
}

fn update_foodspawner(
    mut commands: Commands,
    foods: Query<&Food>,
    segments: Query<&Position, With<Segment>>,
    sheets: Res<SpriteSheetAssets>,
    mut spawners: Query<&mut FoodSpawner>,
    time: Res<Time>,
) {
    if !foods.is_empty() {
        return;
    }

    if let Some(mut spawner) = spawners.iter_mut().next() {
        if !spawner.timer.tick(time.delta()).finished() {
            return;
        } else {
            spawner.timer.reset();
        }
    } else {
        return;
    }

    if segments.iter().count() >= (GRID_SIZE as usize).pow(2) {
        return;
    }

    let mut pos = Position::rnd();
    while segments.iter().any(|p| *p == pos) {
        pos = Position::rnd();
    }

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture_atlas: sheets.food.clone(),
            transform: Transform::from_translation(Vec3::Z),
            ..Default::default()
        })
        .insert(Animation {
            count: FOOD_ANIM_CNT,
        })
        .insert(Food::default())
        .insert(GameComponent)
        .insert(pos);
}

fn update_head_color(mut query: Query<(&mut Head, &mut TextureAtlasSprite)>) {
    for (mut head, mut sprite) in query.iter_mut() {
        let mut src: Vec4 = sprite.color.into();
        let dst: Vec4 = head.color.into();
        let dir = Vec4::normalize(dst - src);
        if dir.is_finite() {
            src += HEAD_COLOR_SPEED * dir
        };

        if dst.distance(src) < HEAD_COLOR_SPEED {
            head.color = Color::hsl(random::<f32>() * 360.0, HEAD_COLOR_S, HEAD_COLOR_L);
        }

        sprite.color = src.into();
    }
}

fn update_head_dir(mut query: Query<(&Head, &mut TextureAtlasSprite, &mut Transform)>) {
    for (head, mut sprite, mut tf) in query.iter_mut() {
        sprite.flip_x = head.dir.flip_x();
        sprite.flip_y = head.dir.flip_y();
        tf.rotation = Quat::from_rotation_z(head.dir.angle());
    }
}

fn update_head_position(
    audio: Res<Audio>,
    mut client: Client<Protocol, DefaultChannels>,
    mut commands: Commands,
    food: Query<(Entity, &Position), (With<Food>, Without<Segment>)>,
    mut heads: Query<(Entity, &mut Head)>,
    player: Res<Player>,
    mut positions: Query<(&mut Position, &Segment), (With<Segment>, Without<Food>)>,
    mut scores: Query<&mut Score>,
    sheets: Res<SpriteSheetAssets>,
    sounds: Res<AudioAssets>,
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
    mut tracks: Query<&mut AudioTrack>,
) {
    if let Some((head_ent, mut head)) = heads.iter_mut().next() {
        if !head.timer.tick(time.delta()).just_finished() {
            return;
        }

        let mut seg_pos = positions
            .iter()
            .map(|(p, s)| (*p, *s))
            .collect::<Vec<(Position, Segment)>>();
        seg_pos.sort_by(|(_, a), (_, b)| a.index.cmp(&b.index));

        for (mut pos, seg) in positions.iter_mut() {
            if seg.index > 0 {
                pos.clone_from(&seg_pos[(seg.index - 1) as usize].0);
            }
        }

        {
            let (mut head_pos, _) = positions.get_mut(head_ent).unwrap();
            match head.dir {
                Direction::Down => head_pos.y = head_pos.y.checked_sub(1).unwrap_or(GRID_SIZE - 1),
                Direction::Left => head_pos.x = head_pos.x.checked_sub(1).unwrap_or(GRID_SIZE - 1),
                Direction::Right => head_pos.x = (head_pos.x + 1) % GRID_SIZE,
                Direction::Up => head_pos.y = (head_pos.y + 1) % GRID_SIZE,
            }
        }

        let count = seg_pos.len();
        let (head_pos, _) = positions.get(head_ent).unwrap();
        let last = seg_pos[(count - 1) as usize].0;
        for (food_ent, pos) in food.iter() {
            if *pos == *head_pos {
                let mut score = scores.iter_mut().next().unwrap();
                score.count += 1;
                audio.play(sounds.game_eat.clone());
                commands.entity(food_ent).despawn();
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: 0,
                            custom_size: Some(Vec2::ONE),
                            ..Default::default()
                        },
                        texture_atlas: sheets.diamond.clone(),
                        transform: Transform::from_translation(Vec3::Z),
                        ..Default::default()
                    })
                    .insert(Animation {
                        count: SEGMENT_ANIM_CNT,
                    })
                    .insert(GameComponent)
                    .insert(Segment { index: count })
                    .insert(last);

                let dur = head.timer.duration().mul_f32(0.95);
                head.timer.set_duration(dur);

                if let Ok(mut track) = tracks.get_single_mut() {
                    track.rate = track.rate * 1.005;
                    audio.set_playback_rate(track.rate);
                }
            }
        }

        for (pos, seg) in positions.iter() {
            if seg.index != 0 && *pos == *head_pos {
                if let Ok(score) = scores.get_single() {
                    client.send_message(
                        DefaultChannels::UnorderedReliable,
                        &HighScore::new(player.name.clone(), score.count),
                    );
                }

                state.set(AppState::Gameover).unwrap();
            }
        }
    }
}

fn update_positions(mut positions: Query<(&Position, &mut Transform)>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let blk = get_blocksize(f32::min(wnd.height(), wnd.width()));
    for (pos, mut tf) in positions.iter_mut() {
        tf.translation = transform_position(blk, *pos).extend(tf.translation.z);
        tf.scale = Vec3::new(blk, blk, tf.scale.z);
    }
}

fn update_scorebar(mut bars: Query<&mut Style, With<ScoreBar>>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    let blk = get_blocksize(wnd_sze);
    let mut style = bars.iter_mut().next().unwrap();
    style.size = Size::new(Val::Px(blk * GRID_SIZE as f32), Val::Px(blk));
}

fn update_scorebar_container(
    mut containers: Query<&mut Style, With<ScoreBarContainer>>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    let blk = get_blocksize(wnd_sze);
    let mut style = containers.iter_mut().next().unwrap();
    style.size = Size::new(
        Val::Px(blk * GRID_SIZE as f32),
        Val::Px(blk + 0.5 * (wnd.height() - wnd_sze)),
    );
}

fn update_scorecoin(mut coins: Query<&mut Style, With<Coin>>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    let blk = get_blocksize(wnd_sze);
    let mut style = coins.iter_mut().next().unwrap();
    style.size = Size::new(Val::Px(blk), Val::Px(blk));
}

fn update_scoretext(
    player: Res<Player>,
    mut texts: Query<(&Score, &mut Text)>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    let blk = get_blocksize(wnd_sze);
    let (score, mut txt) = texts.iter_mut().next().unwrap();
    txt.sections[0].style.font_size = blk;
    txt.sections[0].value = format!("X{} {}", score.count, player.name);
}

fn update_sheets(mut query: Query<(&Animation, &mut TextureAtlasSprite)>, time: Res<Time>) {
    let cnt = 10.0 * time.seconds_since_startup();
    for (a, mut sheet) in query.iter_mut() {
        sheet.index = cnt as usize % a.count;
    }
}
