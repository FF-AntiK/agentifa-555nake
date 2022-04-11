use std::time::Duration;

use agentifa_555nake_protocol::protocol::Protocol;
use bevy::{
    log::LogPlugin,
    prelude::{info, App, Commands, EventReader, Res, ResMut},
    MinimalPlugins,
};
use highscore::{HighScoreList, HighScorePlugin};
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Plugin as ServerPlugin, RoomKey, Server, ServerAddrs, ServerConfig, Stage,
};
use naia_shared::SharedConfig;
use obfstr::obfstr;

mod highscore;

const SRV_ADDR: &str = "127.0.0.1";

#[cfg(debug_assertions)]
const SRV_ADDR_PUB: &str = SRV_ADDR;
#[cfg(not(debug_assertions))]
const SRV_ADDR_PUB: &str = env!("SRV_ADDR");

const SRV_PORT: &str = "55500";
const SRV_PORT_WRTC: &str = "55501";

#[cfg(debug_assertions)]
const SRV_PROT: &str = "http";
#[cfg(not(debug_assertions))]
const SRV_PROT: &str = env!("SRV_PROT");

#[cfg(not(debug_assertions))]
const SRV_KEY: &str = env!("SRV_KEY");
#[cfg(debug_assertions)]
const SRV_KEY: &str = "SRV_KEY";

struct Global {
    main_room_key: RoomKey,
}

fn authorize(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth_message)) = event {
            let key = auth_message.key.get();
            if key == obfstr!(SRV_KEY) {
                server.accept_connection(&user_key);
            } else {
                server.reject_connection(&user_key);
            }
        }
    }
}

fn connect<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    mut server: Server<'world, 'state, Protocol>,
    global: Res<Global>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;
        let address = server
            .user_mut(&user_key)
            .enter_room(&global.main_room_key)
            .address();

        info!("Naia Server connected to: {}", address);
    }
}

fn disconnect(mut event_reader: EventReader<DisconnectionEvent>) {
    for event in event_reader.iter() {
        let DisconnectionEvent(_, user) = event;
        info!("Naia Server disconnected from: {:?}", user.address);
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HighScorePlugin)
        .add_plugin(LogPlugin)
        .add_plugin(ServerPlugin::new(
            ServerConfig::default(),
            SharedConfig::new(Protocol::load(), Some(Duration::from_millis(50)), None),
        ))
        .add_startup_system(setup)
        .add_system_to_stage(Stage::ReceiveEvents, authorize)
        .add_system_to_stage(Stage::ReceiveEvents, connect)
        .add_system_to_stage(Stage::ReceiveEvents, disconnect)
        .add_system_to_stage(Stage::ReceiveEvents, message)
        .add_system_to_stage(Stage::Tick, tick)
        .run();
}

fn message(
    mut event_reader: EventReader<MessageEvent<Protocol>>,
    mut highscore: ResMut<HighScoreList>,
) {
    for event in event_reader.iter() {
        if let MessageEvent(_, Protocol::HighScore(msg)) = event {
            highscore.insert(msg.name.get().clone(), *msg.score.get());
        }
    }
}

fn setup(mut commands: Commands, mut server: Server<Protocol>) {
    server.listen(ServerAddrs::new(
        format!("{}:{}", SRV_ADDR, SRV_PORT).parse().unwrap(),
        format!("{}:{}", SRV_ADDR, SRV_PORT_WRTC).parse().unwrap(),
        &format!("{}://{}:{}", SRV_PROT, SRV_ADDR_PUB, SRV_PORT_WRTC),
    ));

    commands.insert_resource(Global {
        main_room_key: server.make_room().key(),
    });
}

fn tick(mut server: Server<Protocol>) {
    for (_, user_key, entity) in server.scope_checks() {
        server.user_scope(&user_key).include(&entity);
    }

    server.send_all_updates();
}
