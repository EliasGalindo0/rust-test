mod game_data;
mod data_reader;
mod game_data_handler;

use data_reader::DataReader;
use game_data_handler::GameDataHandler;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "src/qgames.txt";

    let games = DataReader::read_game_data(file_path)?;
    GameDataHandler::print_game_data(&games);

    Ok(())
}
