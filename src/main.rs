mod game_data;
mod game_data_reader;
mod game_data_processor;

use game_data_reader::GameDataReader;
use game_data_processor::GameDataProcessor;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "src/qgames.txt";

    let games = GameDataReader::read_game_data(file_path)?;
    GameDataProcessor::print_game_data(&games);

    Ok(())
}
