use crate::game_data::GameData;
use std::collections::HashMap;

pub struct GameDataProcessor;

impl GameDataProcessor {
    pub fn print_game_data(games: &HashMap<i32, GameData>) {
        let mut game_numbers: Vec<i32> = games.keys().cloned().collect();
        game_numbers.sort();

        for game_number in game_numbers {
            let data = &games[&game_number];
            println!("Game {}:", game_number);
            println!("Total Kills: {}", data.total_kills);
            println!("Players:");
            for player in &data.players {
                println!("- {}: {} kills", player, data.kills.get(player).unwrap_or(&0));
            }
            println!("Deaths by means:");
            for (mod_, count) in &data.kills_by_means {
                println!("  {}: {}", mod_, count);
            }
            println!();
        }
    }
}
