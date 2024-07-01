use crate::game_data::GameData;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct DataReader;

impl DataReader {
    pub fn read_game_data(file_path: &str) -> io::Result<HashMap<i32, GameData>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let re_kill = Regex::new(r"(\d+:\d+) Kill: (\d+) (\d+) (\d+): (.*) killed (.*) by (MOD_\w+)").unwrap();
        let mut games = HashMap::new();
        let mut current_game = 0;

        for line in reader.lines() {
            let line = line?;

            if line.contains("InitGame:") {
                current_game += 1;
                games.insert(current_game, GameData::new());
                continue;
            }

            if let Some(caps) = re_kill.captures(&line) {
                let killer = &caps[5];
                let victim = &caps[6];
                let means = &caps[7];

                if let Some(game) = games.get_mut(&current_game) {
                    game.add_kill(killer, victim, means);
                }
            }
        }

        Ok(games)
    }
}
