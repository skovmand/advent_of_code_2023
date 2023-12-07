//! [`Day 2`]
//! TBD

use advent_of_code_2023::{
    days::day02::{how_many_games_would_be_possible, minimum_cube_count_per_game, parse_games},
    print_solution, read_from_stdin,
};

fn main() {
    let input = read_from_stdin();
    let parsed_games = parse_games(&input);

    print_solution(
        2,
        1,
        "What is the sum of the IDs of those games?",
        how_many_games_would_be_possible(&parsed_games).to_string(),
    );

    print_solution(
        2,
        2,
        "What is the sum of the power of these sets?",
        minimum_cube_count_per_game(&parsed_games).to_string(),
    );
}
