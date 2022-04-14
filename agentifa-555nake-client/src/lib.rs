use std::time::Duration;

use agentifa_555nake_protocol::protocol::{Auth, Protocol};
use bevy::{
    asset::{AssetServer, Assets, HandleUntyped},
    ecs::world::World,
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion},
    },
    math::Vec2,
    pbr::StandardMaterial,
    prelude::{
        info, App, ClearColor, Color, EventReader, Handle, Image, ParallelSystemDescriptorCoercion,
        ResMut, State, SystemLabel,
    },
    sprite::TextureAtlas,
    text::Font,
    window::{WindowDescriptor, WindowMode, WindowResizeConstraints, Windows},
    DefaultPlugins,
};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::{AudioPlugin, AudioSource};
use game::GamePlugin;
use gameover::GameOverPlugin;
use load::LoadPlugin;
use menu::MenuPlugin;
use naia_bevy_client::{Client, ClientConfig, Plugin as ClientPlugin, Stage};
use naia_shared::SharedConfig;
use obfstr::obfstr;
use register::RegisterPlugin;
use vkeyboard::VKeyboardPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

mod game;
mod gameover;
mod load;
mod menu;
mod register;
mod vkeyboard;

#[cfg(debug_assertions)]
const SRV_ADDR: &str = "127.0.0.1";
#[cfg(not(debug_assertions))]
const SRV_ADDR: &str = env!("SRV_ADDR");

#[cfg(not(debug_assertions))]
const SRV_KEY: &str = env!("SRV_KEY");
#[cfg(debug_assertions)]
const SRV_KEY: &str = "SRV_KEY";

#[cfg(debug_assertions)]
const SRV_PORT: &str = "55500";
#[cfg(not(debug_assertions))]
const SRV_PORT: &str = env!("SRV_PORT");

#[cfg(debug_assertions)]
const SRV_PROT: &str = "http";
#[cfg(not(debug_assertions))]
const SRV_PROT: &str = env!("SRV_PROT");

const WND_CLR: Color = Color::BLACK;
const WND_TTL: &str = "AGENTIFA 555NAKÉ!";
const WND_SZE_MIN_X: f32 = 200.0;
const WND_SZE_MIN_Y: f32 = 220.0;
const WND_SZE_X: f32 = 600.0;
const WND_SZE_Y: f32 = 660.0;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum AppState {
    Connect,
    Gameover,
    Load,
    Menu,
    MultiPlayer,
    Register,
    SinglePlayer,
}

#[derive(AssetCollection)]
struct AudioAssets {
    #[asset(path = "audio/eat.ogg")]
    game_eat: Handle<AudioSource>,
    #[asset(path = "audio/gameover.ogg")]
    game_over: Handle<AudioSource>,
    #[asset(path = "audio/music_game.ogg")]
    game_music: Handle<AudioSource>,
    #[asset(path = "audio/music_menu.ogg")]
    menu_music: Handle<AudioSource>,
    #[asset(path = "audio/music_menu_intro.ogg")]
    menu_music_intro: Handle<AudioSource>,
}

#[derive(AssetCollection)]
struct FontAssets {
    #[asset(path = "font/RobotoMono-Bold.ttf")]
    bold: Handle<Font>,
    #[asset(path = "font/NotoEmoji-Regular.ttf")]
    emoji: Handle<Font>,
    #[asset(path = "font/RobotoMono-Regular.ttf")]
    regular: Handle<Font>,
}

#[derive(AssetCollection)]
struct ImageAssets {
    #[asset(path = "image/impfliebe.png")]
    bg_game: Handle<Image>,

    #[asset(path = "image/oink.png")]
    bg_menu: Handle<Image>,

    #[asset(path = "image/diamond_still.png")]
    diamond: Handle<Image>,

    #[asset(path = "image/gameover.png")]
    game_over: Handle<Image>,

    #[asset(path = "image/load.png")]
    load: Handle<Image>,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemLabel)]
enum InputState {
    Keyboard,
    Mouse,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum NetState {
    Offline,
    Online,
}

#[derive(Default)]
struct Player {
    name: String,
}

#[derive(AssetCollection)]
struct SpriteSheetAssets {
    /*#[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 6, rows = 1))]
    #[asset(path = "image/diamond.png")]
    diamond: Handle<TextureAtlas>,*/
    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 7, rows = 1))]
    #[asset(path = "image/easteregg.png")]
    easteregg: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 8, rows = 1))]
    #[asset(path = "image/brocc.png")]
    food: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 32.0, tile_size_y = 32.0, columns = 10, rows = 17))]
    #[asset(path = "image/keys.png")]
    keys: Handle<TextureAtlas>,

    #[asset(texture_atlas(
        tile_size_x = 260.0,
        tile_size_y = 260.0,
        padding_x = 1.0,
        columns = 4,
        rows = 1
    ))]
    #[asset(path = "image/pimmler.png")]
    pimmler: Handle<TextureAtlas>,
}

fn connect(client: Client<Protocol>, mut net_state: ResMut<State<NetState>>) {
    info!("Client connected to: {}", client.server_address());
    if vec![NetState::Offline].contains(net_state.current()) {
        net_state.set(NetState::Online).unwrap();
    }
}

fn disconnect(client: Client<Protocol>, mut net_state: ResMut<State<NetState>>) {
    info!("Client disconnected from: {}", client.server_address());
    if vec![NetState::Online].contains(net_state.current()) {
        net_state.set(NetState::Offline).unwrap();
    }
}

fn input_keyboard(
    mut input: EventReader<KeyboardInput>,
    mut state: ResMut<InputState>,
    mut windows: ResMut<Windows>,
) {
    if input.iter().count() > 0 && !vec![InputState::Keyboard].contains(&state) {
        let wnd = windows.get_primary_mut().unwrap();
        wnd.set_cursor_visibility(false);
        *state = InputState::Keyboard;
    }
}

fn input_mouse(
    mut input_button: EventReader<MouseButtonInput>,
    mut input_motion: EventReader<MouseMotion>,
    mut state: ResMut<InputState>,
    mut windows: ResMut<Windows>,
) {
    if (input_button.iter().count() > 0 || input_motion.iter().count() > 0)
        && !vec![InputState::Mouse].contains(&state)
    {
        let wnd = windows.get_primary_mut().unwrap();
        wnd.set_cursor_visibility(true);
        *state = InputState::Mouse;
    }
}

fn setup(mut client: Client<Protocol>) {
    client.auth(Auth::new(obfstr!(SRV_KEY)));
    client.connect(&format!("{}://{}:{}", SRV_PROT, SRV_ADDR, SRV_PORT));
}

#[wasm_bindgen]
pub fn start() {
    let mut app = App::new();

    AssetLoader::new(AppState::Load)
        .continue_to_state(AppState::Connect)
        .with_collection::<AudioAssets>()
        .with_collection::<FontAssets>()
        .with_collection::<ImageAssets>()
        .with_collection::<SpriteSheetAssets>()
        .build(&mut app);

    app.insert_resource(ClearColor(WND_CLR))
        .insert_resource(InputState::Mouse)
        .insert_resource(Player::default())
        .insert_resource(WindowDescriptor {
            height: WND_SZE_Y,
            mode: WindowMode::Windowed,
            resize_constraints: WindowResizeConstraints {
                min_height: WND_SZE_MIN_Y,
                min_width: WND_SZE_MIN_X,
                ..Default::default()
            },
            title: WND_TTL.to_string(),
            width: WND_SZE_X,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(ClientPlugin::new(
            ClientConfig::default(),
            SharedConfig::new(Protocol::load(), Some(Duration::from_millis(50)), None),
        ))
        .add_plugin(GamePlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(LoadPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(RegisterPlugin)
        .add_plugin(VKeyboardPlugin)
        .add_startup_system(setup)
        .add_state(AppState::Load)
        .add_state(NetState::Offline)
        .add_system(input_keyboard.label(InputState::Keyboard))
        .add_system(input_mouse.label(InputState::Mouse))
        .add_system_to_stage(Stage::Connection, connect)
        .add_system_to_stage(Stage::Disconnection, disconnect)
        .run();
}
