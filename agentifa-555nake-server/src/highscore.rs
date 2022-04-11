use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
};

use agentifa_555nake_protocol::protocol::{HighScore, HighScoreRank, Protocol};
use bevy::prelude::{Commands, Entity, Plugin, Query, Res, ResMut};
use frank::rank_dense_greater;
use naia_bevy_server::Server;

use crate::Global;

const HIGHSCORE: &str = "highscore.json";

pub struct HighScoreList {
    entities: HashMap<String, Entity>,
    entries: HashMap<String, usize>,
    ranks: Vec<(String, usize)>,
}

impl HighScoreList {
    pub fn insert(&mut self, name: String, score: usize) {
        self.entries
            .entry(name)
            .and_modify(|s| *s = (*s).max(score))
            .or_insert(score);

        self.ranks.clear();
        for (name, score) in self.entries.iter() {
            self.ranks.push((name.clone(), *score));
        }

        self.ranks.sort_by_key(|(_, s)| *s);
        self.ranks.reverse();

        let ranks: Vec<usize> = self.ranks.iter().map(|(_, s)| *s).collect();
        let ranks = rank_dense_greater(&ranks);
        for (i, (_, r)) in self.ranks.iter_mut().enumerate() {
            *r = ranks[i];
        }

        if let Ok(file) = File::create(HIGHSCORE) {
            let writer = BufWriter::new(file);
            let _ = serde_json::to_writer(writer, &self.entries);
        }
    }

    fn new() -> Self {
        let mut store: HashMap<String, usize> = HashMap::new();
        if let Ok(file) = File::open(HIGHSCORE) {
            let reader = BufReader::new(file);
            if let Ok(list) = serde_json::from_reader(reader) {
                store = list;
            }
        }

        let entities = HashMap::new();
        let entries = HashMap::new();
        let ranks = Vec::new();
        let mut highscore = HighScoreList {
            entities,
            entries,
            ranks,
        };

        for (name, score) in store.iter() {
            highscore.insert(name.clone(), *score);
        }

        highscore
    }
}

pub struct HighScorePlugin;

impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            .add_system(update_highscore)
            .add_system(update_ranks);
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(HighScoreList::new());
}

fn update_highscore(
    global: Res<Global>,
    mut list: ResMut<HighScoreList>,
    mut query: Query<&mut HighScore>,
    mut server: Server<Protocol>,
) {
    let mut new_entities = HashMap::new();
    for (name, score) in list.entries.iter() {
        if let Some(entity) = list.entities.get(name) {
            let mut hs = query.get_mut(*entity).unwrap();
            hs.score.set(*list.entries.get(name).unwrap());
        } else {
            new_entities.insert(
                name.clone(),
                server
                    .spawn()
                    .enter_room(&global.main_room_key)
                    .insert(HighScore::new(name.clone(), *score))
                    .id(),
            );
        }
    }

    for (name, entity) in new_entities.iter() {
        list.entities.insert(name.clone(), *entity);
    }
}

fn update_ranks(
    list: Res<HighScoreList>,
    mut query: Query<&mut HighScoreRank>,
    mut server: Server<Protocol>,
) {
    for (position, (name, rank)) in list.ranks.iter().enumerate() {
        if let Some(entity) = list.entities.get(name) {
            if let Ok(mut r) = query.get_mut(*entity) {
                r.position.set(position);
                r.rank.set(*rank);
            } else {
                server
                    .entity_mut(entity)
                    .insert(HighScoreRank::new(position, *rank));
            }
        }
    }
}
