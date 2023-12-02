//! [`Day 1`]
//! Part 1: Simple task solved in 10 minutes before breakfast.
//! Part 2: Frustrating. The requirement to allow "shared letters" was not enforced
//!         in the test data. Not very happy with my solution, but it did solve it.
//!
//! [`Day 1`]: https://adventofcode.com/2023/day/1

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("Hello, world!");
}

fn sum_of_calibration_values(input_lines: &str) -> u64 {
    input_lines
        .lines()
        .map(find_first_and_last_digit)
        .map(|numeric_string| {
            numeric_string
                .parse::<u64>()
                .expect("parse combined string")
        })
        .sum()
}

fn find_first_and_last_digit(line: &str) -> String {
    let numeric_chars = line.chars().filter(|c| c.is_numeric()).collect::<String>();

    let first_char = numeric_chars.chars().next().expect("1st char");
    let last_char = numeric_chars.chars().last().expect("last char");

    format!("{first_char}{last_char}")
}

fn sum_of_calibration_values_allowing_strings(input_lines: &str) -> u64 {
    input_lines
        .lines()
        .map(input_to_numbers)
        .map(|string| find_first_and_last_digit(string.as_str()))
        .map(|numeric_string| {
            numeric_string
                .parse::<u64>()
                .expect("parse combined string")
        })
        .sum()
}

lazy_static! {
    static ref PREFIXES: Vec<(&'static str, &'static str)> = vec![
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("one", "1"),
        ("ne", "1"),
        ("two", "2"),
        ("wo", "2"),
        ("three", "3"),
        ("hree", "3"),
        ("four", "4"),
        ("our", "4"),
        ("five", "5"),
        ("ive", "5"),
        ("six", "6"),
        ("ix", "6"),
        ("seven", "7"),
        ("even", "7"),
        ("eight", "8"),
        ("ight", "8"),
        ("nine", "9"),
        ("ine", "9"),
    ];
}

fn input_to_numbers(mut line: &str) -> String {
    let mut matched: Vec<String> = Vec::new();

    'line_search: while !line.is_empty() {
        // Match a prefix, when a prefix is matched, start over
        for (prefix, value) in PREFIXES.iter() {
            if let Some(rest) = line.strip_prefix(prefix) {
                matched.push(value.to_string());
                line = rest;

                // Continue from the top of the while statement
                continue 'line_search;
            }
        }

        line = &line[1..];
    }

    matched.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "t1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const EXAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day01.txt");

    #[test]
    fn solves_the_first_example() {
        assert_eq!(sum_of_calibration_values(EXAMPLE_INPUT_1), 142);
    }

    #[test]
    fn solves_part_1() {
        assert_eq!(sum_of_calibration_values(PUZZLE_INPUT), 54927);
    }

    #[test]
    fn solves_the_second_example() {
        assert_eq!(
            sum_of_calibration_values_allowing_strings(EXAMPLE_INPUT_2),
            281
        );
    }

    #[test]
    fn solves_part_2() {
        assert_eq!(
            sum_of_calibration_values_allowing_strings(PUZZLE_INPUT),
            54581
        );
    }

    #[test]
    fn input_to_numbers_unit_test() {
        assert_eq!(input_to_numbers("14gxqgqsqqbxfpxnbccjc33eight"), "14338");
        assert_eq!(input_to_numbers("eight2sevenkl"), "827");
        assert_eq!(input_to_numbers("mrjstg5onetwoeightgcczx8vgrgl"), "51288");
        assert_eq!(input_to_numbers("9246"), "9246");
        assert_eq!(
            input_to_numbers("ninetwo2crrqk2grsctqxqbcrmrdsqbrz9eight"),
            "922298"
        );

        assert_eq!(input_to_numbers("five6dlhx1"), "561");
        assert_eq!(input_to_numbers("29qhsdqqtgrk4"), "294");
        assert_eq!(input_to_numbers("mjgtrjnlttxjlsixsix5"), "665");
        assert_eq!(
            input_to_numbers("bgnnvfsnbpx29vsjrlmgmsqthreeqxvclkhlv"),
            "293"
        );
        assert_eq!(input_to_numbers("sevennine3gsmxncqlqvfktxrtcone"), "7931");
        assert_eq!(input_to_numbers("2mkzrqlmhsveight1sjtwo2"), "28122");
        assert_eq!(input_to_numbers("rrrbvnrvnqfdh86"), "86");

        assert_eq!(input_to_numbers("94threeqrhkcpkeightfour"), "94384");
    }
}
