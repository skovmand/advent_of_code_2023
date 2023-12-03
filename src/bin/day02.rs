//! [`Day 2`]
//! TBD

use std::collections::HashMap;

use advent_of_code_2023::{print_solution, read_from_stdin};

fn main() {
    let input = read_from_stdin();

    print_solution(
        2,
        1,
        "What is the sum of the IDs of those games?",
        find_sum_of_possible_game_ids(&input).to_string(),
    );

    print_solution(
        2,
        2,
        "What is the sum of the power of these sets?",
        find_sum_of_products_of_minimum_cube_count(&input).to_string(),
    );
}

// Part 1
fn find_sum_of_possible_game_ids(input: &str) -> u64 {
    let bag = Bag {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = parse_games(input);

    games
        .iter()
        .filter(|game| game.draws.iter().all(|draw| bag.is_game_possible(draw)))
        .map(|game| game.id)
        .sum()
}

// Part 2
fn find_sum_of_products_of_minimum_cube_count(input: &str) -> u64 {
    let games = parse_games(input);

    games
        .iter()
        .map(|game| {
            // Defaulting to 1 because we need the product afterwards
            let max_red = game.draws.iter().map(|draw| draw.red).max().unwrap_or(1);
            let max_green = game.draws.iter().map(|draw| draw.green).max().unwrap_or(1);
            let max_blue = game.draws.iter().map(|draw| draw.blue).max().unwrap_or(1);

            max_red * max_green * max_blue
        })
        .sum()
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect::<Vec<Game>>()
}

fn parse_game(line: &str) -> Game {
    let number_and_draw = line.strip_prefix("Game ").expect("no game");
    let mut splits = number_and_draw.split(": ");

    let game_id = {
        let game_id_string = splits.next().expect("no number");

        game_id_string
            .parse::<u64>()
            .expect("could not parse game id")
    };

    let draws = {
        let draw_string = splits.next().expect("no draw");
        parse_draws(draw_string)
    };

    Game { id: game_id, draws }
}

fn parse_draws(draws_as_string: &str) -> Vec<Draw> {
    draws_as_string
        .split("; ")
        .map(parse_single_draw)
        .map(|draw| Draw {
            red: *draw.get("red").unwrap_or(&0),
            green: *draw.get("green").unwrap_or(&0),
            blue: *draw.get("blue").unwrap_or(&0),
        })
        .collect()
}

fn parse_single_draw(draw_string: &str) -> HashMap<String, u64> {
    draw_string.split(", ").map(parse_single_color).collect()
}

fn parse_single_color(color_string: &str) -> (String, u64) {
    let mut splits = color_string.split(' ');

    let amount = {
        let raw = splits.next().expect("no amount");
        raw.parse::<u64>().expect("can't parse amount")
    };

    let color = {
        let color_string = splits.next().expect("no color");
        assert!(["red", "green", "blue"].contains(&color_string));
        color_string
    };

    (color.to_string(), amount)
}

#[derive(Debug)]
struct Game {
    id: u64,
    draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    red: u64,
    green: u64,
    blue: u64,
}

struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    fn is_game_possible(&self, draw: &Draw) -> bool {
        draw.red <= self.red && draw.blue <= self.blue && draw.green <= self.green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day02.txt");

    #[test]
    fn solves_p1_example() {
        assert_eq!(find_sum_of_possible_game_ids(EXAMPLE_DATA), 8);
    }

    #[test]
    fn solves_p1() {
        assert_eq!(find_sum_of_possible_game_ids(PUZZLE_INPUT), 1734);
    }

    #[test]
    fn solved_p2_example() {
        assert_eq!(
            find_sum_of_products_of_minimum_cube_count(EXAMPLE_DATA),
            2286
        );
    }

    #[test]
    fn solves_p2() {
        assert_eq!(
            find_sum_of_products_of_minimum_cube_count(PUZZLE_INPUT),
            70387
        );
    }
}
