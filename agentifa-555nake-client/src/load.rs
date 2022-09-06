use std::f32::consts::PI;

use bevy::{
    ecs::schedule::ShouldRun,
    math::{Quat, Vec2},
    prelude::{
        App, AssetServer, BuildChildren, Camera2dBundle, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Handle, Image, In, IntoChainSystem, NodeBundle, Plugin, Query,
        Res, ResMut, State, SystemSet, TextBundle, Transform, UiCameraConfig, With,
    },
    sprite::SpriteBundle,
    text::{Font, Text, TextStyle},
    time::{FixedTimestep, Time},
    ui::{AlignItems, JustifyContent, Size, Style, Val},
    window::Windows,
};

use crate::{AppState, FontAssets, ImageAssets, NetState};

const CONTXT: &str = "Verbinde";
const LOADCLR: Color = Color::CYAN;
const LOADFNT: &str = "font/RobotoMono-Bold.ttf";
const LOADIMG: &str = "image/load.png";
const LOADSZE: f32 = 64.0; // Font size
const LOADTXT: &str = "Lade";
const MARQUEE: [&str; 3] = [".  ", " . ", "  ."];
const MARQUEE_SPEED: f64 = 1.0; // Seconds per step
const ROTATION_SPEED: f32 = PI * 0.2; // Angle per Second

pub struct LoadPlugin;
impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        let with_load_systems =
            |s: SystemSet| -> SystemSet { s.with_system(rotate).with_system(transform) };

        let with_connect_systems =
            |s: SystemSet| -> SystemSet { with_load_systems(s).with_system(connect) };

        app.add_system_set(SystemSet::on_enter(AppState::Connect).with_system(setup_connect))
            .add_system_set(SystemSet::on_exit(AppState::Connect).with_system(cleanup))
            .add_system_set(with_connect_systems(SystemSet::on_update(
                AppState::Connect,
            )))
            .add_system_set(SystemSet::on_enter(AppState::Load).with_system(setup_load))
            .add_system_set(SystemSet::on_exit(AppState::Load).with_system(cleanup))
            .add_system_set(with_load_systems(SystemSet::on_update(AppState::Load)))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(MARQUEE_SPEED).chain(
                        |In(input): In<ShouldRun>, state: Res<State<AppState>>| {
                            match state.current() {
                                AppState::Connect => input,
                                AppState::Load => input,
                                _ => ShouldRun::No,
                            }
                        },
                    ))
                    .with_system(update_marquee),
            );
    }
}

#[derive(Component)]
struct LoadComponent;

#[derive(Component, Default)]
struct Rotation {
    angle: f32,
}

#[derive(Component, Default)]
struct Marquee {
    step: usize,
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn connect(mut app_state: ResMut<State<AppState>>, net_state: Res<State<NetState>>) {
    if vec![NetState::Online].contains(net_state.current()) {
        app_state.set(AppState::Menu).unwrap();
    }
}

fn rotate(mut query: Query<&mut Rotation>, time: Res<Time>) {
    for mut rotation in query.iter_mut() {
        rotation.angle += time.delta().as_secs_f32() * ROTATION_SPEED;
    }
}

fn setup(cmd: &mut Commands, fnt: Handle<Font>, img: Handle<Image>, txt: &str) {
    cmd.spawn_bundle(Camera2dBundle::default())
        .insert(LoadComponent)
        .insert(UiCameraConfig { show_ui: true });

    cmd.spawn_bundle(SpriteBundle {
        sprite: bevy::sprite::Sprite {
            custom_size: Some(Vec2::ONE),
            ..Default::default()
        },
        texture: img,
        ..Default::default()
    })
    .insert(LoadComponent)
    .insert(Rotation::default());

    cmd.spawn_bundle(NodeBundle {
        color: Color::NONE.into(),
        style: Style {
            flex_direction: bevy::ui::FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            size: Size::new(Val::Percent(100.), Val::Percent(100.)),
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(text(fnt.clone(), txt));
    })
    .with_children(|parent| {
        let marquee = Marquee::default();
        parent
            .spawn_bundle(text(fnt.clone(), MARQUEE[marquee.step]))
            .insert(marquee);
    })
    .insert(LoadComponent);
}

fn setup_connect(mut commands: Commands, fonts: Res<FontAssets>, images: Res<ImageAssets>) {
    setup(
        &mut commands,
        fonts.bold.clone(),
        images.load.clone(),
        CONTXT,
    );
}

fn setup_load(assets: Res<AssetServer>, mut commands: Commands) {
    setup(
        &mut commands,
        assets.load(LOADFNT),
        assets.load(LOADIMG),
        LOADTXT,
    );
}

fn text(font: Handle<Font>, value: &str) -> TextBundle {
    TextBundle {
        text: Text::from_section(
            value,
            TextStyle {
                font,
                font_size: LOADSZE,
                color: LOADCLR,
            },
        ),
        ..Default::default()
    }
}

fn transform(mut query: Query<(&Rotation, &mut Transform)>, windows: Res<Windows>) {
    let wnd = windows.get_primary().unwrap();
    let wnd_sze = f32::min(wnd.height(), wnd.width());
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(rotation.angle);
        transform.scale = Vec2::splat(wnd_sze).extend(transform.scale.z);
    }
}

fn update_marquee(mut query: Query<(&mut Marquee, &mut Text)>) {
    for (mut marquee, mut text) in query.iter_mut() {
        marquee.step += 1;
        if marquee.step >= MARQUEE.len() {
            marquee.step = 0;
        }

        text.sections[0].value = format!("{}", MARQUEE[marquee.step]);
    }
}
