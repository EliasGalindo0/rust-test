use std::collections::{HashMap, HashSet};

pub struct GameData {
    pub total_kills: i32,
    pub players: HashSet<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, i32>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            total_kills: 0,
            players: HashSet::new(),
            kills: HashMap::new(),
            kills_by_means: HashMap::new(),
        }
    }

    pub fn add_kill(&mut self, killer: &str, victim: &str, means: &str) {
        self.total_kills += 1;

        if killer != "<world>" {
            self.players.insert(killer.to_string());
            *self.kills.entry(killer.to_string()).or_insert(0) += 1;
        }

        if victim != "<world>" {
            self.players.insert(victim.to_string());
            if killer == "<world>" {
                *self.kills.entry(victim.to_string()).or_insert(0) -= 1;
            }
        }

        *self.kills_by_means.entry(means.to_string()).or_insert(0) += 1;
    }
}
