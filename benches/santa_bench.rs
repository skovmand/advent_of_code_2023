use advent_of_code_2023::days::day02::{
    how_many_games_would_be_possible, minimum_cube_count_per_game, parse_games,
};
use criterion::{criterion_group, criterion_main, Criterion};

const INPUT: &str = include_str!("../puzzle_inputs/day02.txt");

fn criterion_benchmark(c: &mut Criterion) {
    let parsed_games = parse_games(INPUT);

    c.bench_function("fib 20", |b| {
        b.iter(|| how_many_games_would_be_possible(&parsed_games))
    });

    c.bench_function("fib 20", |b| {
        b.iter(|| minimum_cube_count_per_game(&parsed_games))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
