# Game Data Processor

This Rust project reads game data from a `qgames.txt` file, processes the data to calculate various statistics, and prints a detailed report. The project follows SOLID principles to ensure clean and maintainable code.

## Project Structure

The project is divided into several modules:

- **main.rs**: The entry point of the application.
- **game_data.rs**: Contains the `GameData` struct and related methods.
- **game_data_reader.rs**: Responsible for reading and parsing the game data from the file.
- **game_data_processor.rs**: Responsible for processing and printing the game data.

## Prerequisites

To run this project, you need to have Rust installed. You can install Rust using [rustup](https://rustup.rs/).

Additionally, the project depends on the `regex` crate, which is specified in the `Cargo.toml` file.

## Installation

1. **Clone the repository**:

    ```bash
    git clone https://github.com/yourusername/game-data-processor.git
    cd game-data-processor
    ```

2. **Ensure you have Rust installed**:

    ```bash
    rustup install stable
    ```

3. **Build the project**:

    ```bash
    cargo build
    ```

## Usage

1. **Place the `qgames.txt` file in the root directory of the project**:
   
   Make sure the `qgames.txt` file containing the game data is in the same directory as the `Cargo.toml` file.

2. **Run the project**:

    ```bash
    cargo run
    ```

    This will execute the program, read the `qgames.txt` file, process the data, and print a detailed report of each game.

## File Format

The `qgames.txt` file should contain the game data in the following format:

<time> Kill: <killer_id> <victim_id> <mean_of_death>: <killer> killed <victim> by <MOD>

Example:

```bash
0:00 Kill: 1022 3 22: John killed Paul by MOD_ROCKET
0:01 Kill: 1022 4 22: John killed George by MOD_ROCKET
InitGame:
0:02 Kill: 1023 2 22: George killed John by MOD_SHOTGUN
```

## Explanation of Modules

- **GameData**:
  - Struct that represents the data for a single game. It contains fields for total kills, players, kills, and kills by means.
  - Methods for adding kills and updating game statistics.

- **GameDataReader**:
  - Responsible for reading and parsing the game data from the `qgames.txt` file.
  - Uses the `regex` crate to extract information about kills, including the killer, victim, and means of the kill.

- **GameDataProcessor**:
  - Processes the parsed game data and prints a detailed report.

## Example Output

```bash
Game 1:
Total Kills: 2
Players:

John: 2 kills
Paul: -1 kills
George: 1 kills
Deaths by means:
MOD_ROCKET: 2
MOD_SHOTGUN: 1
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
