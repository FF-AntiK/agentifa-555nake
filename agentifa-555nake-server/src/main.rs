use std::{collections::HashMap, time::Duration};

use agentifa_555nake_protocol::protocol::{
    AssignMsg, Direction, Food, Head, Position, Protocol, QuitCmd, Score, Segment, Vincible,
    GRID_SIZE,
};
use bevy::{
    log::LogPlugin,
    prelude::{
        App, Commands, Component, Entity, EventReader, ParallelSystemDescriptorCoercion, Query,
        Res, ResMut, Time, Timer, With, Without,
    },
    MinimalPlugins,
};
use highscore::{HighScoreList, HighScorePlugin};
use naia_bevy_server::{
    events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent, MessageEvent},
    Plugin as ServerPlugin, RoomKey, Server, ServerAddrs, ServerConfig, Stage, UserKey,
};
use naia_shared::{DefaultChannels, SharedConfig};
use obfstr::obfstr;

mod highscore;

const FOOD_SPAWN_DUR: f32 = 10.0;
const HEAD_MOV_DUR_START: f32 = 0.5;
const HEAD_MOV_DUR_FAKTOR: f32 = 0.95;
const SRV_ADDR: &str = "127.0.0.1";
const SRV_PORT: &str = "55500";
const SRV_PORT_WRTC: &str = "55501";
const STARTPOS_X: usize = 5;
const STARTPOS_Y: usize = 5;

#[cfg(debug_assertions)]
const SRV_ADDR_PUB: &str = SRV_ADDR;
#[cfg(not(debug_assertions))]
const SRV_ADDR_PUB: &str = env!("SRV_ADDR");

#[cfg(debug_assertions)]
const SRV_PROT: &str = "http";
#[cfg(not(debug_assertions))]
const SRV_PROT: &str = env!("SRV_PROT");

#[cfg(not(debug_assertions))]
const SRV_KEY: &str = env!("SRV_KEY");
#[cfg(debug_assertions)]
const SRV_KEY: &str = "SRV_KEY";

struct Global {
    food_timer: Timer,
    last_time: Duration,
    main_room_key: RoomKey,
    player_heads: HashMap<UserKey, Entity>,
    player_keys: HashMap<Entity, UserKey>,
    player_segments: HashMap<Entity, UserKey>,
    segment_order: Vec<Entity>,
}

#[derive(Component)]
struct TimerComponent {
    timer: Timer,
}

fn authorize(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth_message)) = event {
            let key = &*auth_message.key;
            if key == obfstr!(SRV_KEY) {
                server.accept_connection(&user_key);
            } else {
                server.reject_connection(&user_key);
            }
        }
    }
}

fn command_message<'world, 'state>(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut global: ResMut<Global>,
    mut query: Query<&mut Head>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        match event {
            MessageEvent(user_key, _, Protocol::DirCmd(msg)) => {
                if let Some(entity) = global.player_heads.get(user_key) {
                    if let Ok(mut head) = query.get_mut(*entity) {
                        *head.dir = *msg.dir;
                        *head.running = true;
                    }
                }
            }
            MessageEvent(user_key, _, Protocol::QuitCmd(_)) => {
                despawn_player(&mut global, &mut server, user_key);
            }
            MessageEvent(user_key, _, Protocol::StartCmd(msg)) => {
                let entity = server
                    .spawn()
                    .enter_room(&global.main_room_key)
                    .insert(Head::new((*msg.name).clone()))
                    .insert(Position::new(STARTPOS_X, STARTPOS_Y))
                    .insert(Score::new())
                    .id();

                commands.entity(entity).insert(TimerComponent {
                    timer: Timer::from_seconds(HEAD_MOV_DUR_START, true),
                });

                global.player_heads.insert(*user_key, entity);
                global.player_keys.insert(entity, *user_key);

                let mut assign_msg = AssignMsg::new();
                assign_msg.entity.set(&server, &entity);
                server.send_message(user_key, DefaultChannels::UnorderedReliable, &assign_msg);
            }
            _ => (),
        }
    }
}

fn connect<'world, 'state>(
    global: Res<Global>,
    mut event_reader: EventReader<ConnectionEvent>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        let ConnectionEvent(user_key) = event;
        server.user_mut(&user_key).enter_room(&global.main_room_key);
    }
}

fn despawn_player<'world, 'state>(
    global: &mut Global,
    server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
    user_key: &UserKey,
) {
    if let Some(entity) = global.player_heads.remove(user_key) {
        global.player_keys.remove(&entity);
        server.entity_mut(&entity).despawn();
    }

    global.player_segments.retain(|k, v| {
        let retain = *v != *user_key;
        if !retain {
            global.segment_order.retain(|e| *e != *k);
            server.entity_mut(k).despawn();
        }

        retain
    });
}

fn disconnect<'world, 'state>(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut global: ResMut<Global>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, _) = event;
        despawn_player(&mut global, &mut server, user_key);
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(HighScorePlugin)
        .add_plugin(LogPlugin)
        .add_plugin(ServerPlugin::<Protocol, DefaultChannels>::new(
            ServerConfig::default(),
            SharedConfig::default(),
        ))
        .add_startup_system(setup)
        .add_system_to_stage(Stage::ReceiveEvents, authorize)
        .add_system_to_stage(Stage::ReceiveEvents, command_message)
        .add_system_to_stage(Stage::ReceiveEvents, connect)
        .add_system_to_stage(Stage::ReceiveEvents, disconnect)
        .add_system_to_stage(Stage::Tick, update_collisions.after(update_foods))
        .add_system_to_stage(Stage::Tick, update_foods.after(update_heads))
        .add_system_to_stage(Stage::Tick, update_heads.after(update_scope))
        .add_system_to_stage(Stage::Tick, update_scope)
        .add_system_to_stage(Stage::Tick, update_server.after(update_collisions))
        .add_system_to_stage(Stage::Tick, update_time.after(update_server))
        .run();
}

fn setup(mut commands: Commands, mut server: Server<Protocol, DefaultChannels>, time: Res<Time>) {
    server.listen(&ServerAddrs::new(
        format!("{}:{}", SRV_ADDR, SRV_PORT).parse().unwrap(),
        format!("{}:{}", SRV_ADDR, SRV_PORT_WRTC).parse().unwrap(),
        &format!("{}://{}:{}", SRV_PROT, SRV_ADDR_PUB, SRV_PORT_WRTC),
    ));

    commands.insert_resource(Global {
        food_timer: Timer::from_seconds(FOOD_SPAWN_DUR, true),
        last_time: time.time_since_startup(),
        main_room_key: server.make_room().key(),
        player_heads: HashMap::new(),
        player_keys: HashMap::new(),
        player_segments: HashMap::new(),
        segment_order: Vec::new(),
    });
}

fn update_collisions(
    mut global: ResMut<Global>,
    heads: Query<&Head>,
    mut highscore: ResMut<HighScoreList>,
    positions: Query<(Entity, &Position), With<Vincible>>,
    scores: Query<&Score>,
    mut server: Server<Protocol, DefaultChannels>,
    vincibles: Query<&Vincible>,
) {
    let mut to_despawn = vec![];
    for (user_key, ent_head) in global.player_heads.iter() {
        if !vincibles.contains(*ent_head) {
            continue;
        }

        let (_, pos) = positions.get(*ent_head).unwrap();
        if positions
            .iter()
            .any(|(e, p)| e != *ent_head && *p.x == *pos.x && *p.y == *pos.y)
        {
            to_despawn.push(*user_key);
        }
    }

    for user_key in to_despawn.iter() {
        let entity = *global.player_heads.get(user_key).unwrap();
        highscore.insert(
            (*heads.get(entity).unwrap().name).clone(),
            *scores.get(entity).unwrap().level,
        );

        despawn_player(&mut global, &mut server, user_key);
        server.send_message(
            user_key,
            DefaultChannels::UnorderedReliable,
            &QuitCmd::new(),
        );
    }
}

fn update_foods(
    mut global: ResMut<Global>,
    positions: Query<&Position>,
    mut server: Server<Protocol, DefaultChannels>,
    time: Res<Time>,
) {
    let delta = time.time_since_startup() - global.last_time;
    if global.food_timer.tick(delta).just_finished() {
        if positions.iter().count() >= GRID_SIZE.pow(2) {
            return;
        }

        let mut position = Position::rnd(GRID_SIZE);
        while positions
            .iter()
            .any(|p| *p.x == *position.x && *p.y == *position.y)
        {
            position = Position::rnd(GRID_SIZE);
        }

        server
            .spawn()
            .enter_room(&global.main_room_key)
            .insert(Food::new())
            .insert(position);
    }
}

fn update_heads(
    mut global: ResMut<Global>,
    foods: Query<(Entity, &Position), (With<Food>, Without<Head>)>,
    mut heads: Query<(
        Entity,
        &Head,
        &mut Score,
        &mut TimerComponent,
        &mut Position,
    )>,
    mut segments: Query<(&mut Position, &mut Segment), (Without<Head>, Without<Food>)>,
    mut server: Server<Protocol, DefaultChannels>,
    time: Res<Time>,
) {
    let delta = time.time_since_startup() - global.last_time;
    let global = &mut *global;

    for (head_ent, head, mut score, mut timer, mut head_pos) in heads.iter_mut() {
        if !*head.running {
            continue;
        }

        if let Some(user_key) = global.player_keys.get(&head_ent) {
            if !timer.timer.tick(delta).just_finished() {
                continue;
            }

            let mut old_pos = Position::new(*head_pos.x, *head_pos.y);
            match *head.dir {
                Direction::Down => *head_pos.y = head_pos.y.checked_sub(1).unwrap_or(GRID_SIZE - 1),
                Direction::Left => *head_pos.x = head_pos.x.checked_sub(1).unwrap_or(GRID_SIZE - 1),
                Direction::Right => *head_pos.x = (*head_pos.x + 1) % GRID_SIZE,
                Direction::Up => *head_pos.y = (*head_pos.y + 1) % GRID_SIZE,
            }

            for entity in global.segment_order.iter() {
                if let Some(key) = global.player_segments.get(entity) {
                    if *key != *user_key {
                        continue;
                    }

                    if let Ok((mut position, mut segment)) = segments.get_mut(*entity) {
                        let new_pos = Position::new(*position.x, *position.y);
                        *position.x = *old_pos.x;
                        *position.y = *old_pos.y;
                        *old_pos.x = *new_pos.x;
                        *old_pos.y = *new_pos.y;
                        *segment.synced = true;
                    }
                }
            }

            if let Some((entity, _)) = foods
                .iter()
                .find(|(_, p)| *p.x == *head_pos.x && *p.y == *head_pos.y)
            {
                server.entity_mut(&entity).despawn();
                let entity = server
                    .spawn()
                    .enter_room(&global.main_room_key)
                    .insert(old_pos)
                    .insert(Segment::new())
                    .insert(Vincible)
                    .id();

                global.player_segments.insert(entity, *user_key);
                global.segment_order.push(entity);

                let dur = timer.timer.duration().mul_f32(HEAD_MOV_DUR_FAKTOR);
                timer.timer.set_duration(dur);
                if *score.level < 1 {
                    server.entity_mut(&head_ent).insert(Vincible);
                }

                *score.level += 1;
            }
        }
    }
}

fn update_scope(mut server: Server<Protocol, DefaultChannels>) {
    for (_, user_key, entity) in server.scope_checks() {
        server.user_scope(&user_key).include(&entity);
    }
}

fn update_server(mut server: Server<Protocol, DefaultChannels>) {
    server.send_all_updates();
}

fn update_time(mut global: ResMut<Global>, time: Res<Time>) {
    global.last_time = time.time_since_startup();
}
