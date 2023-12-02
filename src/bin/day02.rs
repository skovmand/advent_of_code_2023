//! [`Day 2`]
//! TBD

fn main() {
    println!("Hello, world!");
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
        .filter(|game| {
            game.rounds
                .iter()
                .all(|rounds| bag.is_game_possible(rounds))
        })
        .map(|game| game.id)
        .sum()
}

// Part 2
fn find_sum_of_products_of_minimum_cube_count(input: &str) -> u64 {
    let games = parse_games(input);

    games
        .iter()
        .map(|game| {
            let mut max_red = 1;
            let mut max_green = 1;
            let mut max_blue = 1;

            for round in game.rounds.iter() {
                for draw in round.iter() {
                    match draw {
                        Cube::Red(amount) => {
                            if amount > &max_red {
                                max_red = *amount;
                            }
                        }
                        Cube::Green(amount) => {
                            if amount > &max_green {
                                max_green = *amount;
                            }
                        }
                        Cube::Blue(amount) => {
                            if amount > &max_blue {
                                max_blue = *amount;
                            }
                        }
                    }
                }
            }

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

    let rounds = {
        let draw_string = splits.next().expect("no draw");

        draw_string
            .split("; ")
            .map(parse_draws)
            .collect::<Vec<Vec<Cube>>>()
    };

    Game {
        id: game_id,
        rounds,
    }
}

fn parse_draws(set_as_string: &str) -> Vec<Cube> {
    set_as_string
        .split(", ")
        .map(parse_draw)
        .collect::<Vec<Cube>>()
}

fn parse_draw(draw_string: &str) -> Cube {
    let mut splits = draw_string.split(' ');

    let amount = {
        let raw = splits.next().expect("no amount");
        raw.parse::<u64>().expect("can't parse amount")
    };

    let color_string = splits.next().expect("no color");

    match color_string {
        "red" => Cube::Red(amount),
        "green" => Cube::Green(amount),
        "blue" => Cube::Blue(amount),
        _ => panic!("unexpected color string"),
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    rounds: Vec<Vec<Cube>>,
}

#[derive(Debug)]
enum Cube {
    Red(u64),
    Green(u64),
    Blue(u64),
}

struct Bag {
    red: u64,
    green: u64,
    blue: u64,
}

impl Bag {
    fn is_game_possible(&self, desired_draws: &[Cube]) -> bool {
        desired_draws.iter().all(|cube| match cube {
            Cube::Red(amount) => amount <= &self.red,
            Cube::Green(amount) => amount <= &self.green,
            Cube::Blue(amount) => amount <= &self.blue,
        })
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
