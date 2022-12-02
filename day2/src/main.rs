use day2::{rounds_points_calculator_part1, rounds_points_calculator_part2};
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!(
        "Player points after following strategy guide:{:?}",
        rounds_points_calculator_part1(&contents)
    );

    println!(
        "Player points after following strategy guide 2: {:?}",
        rounds_points_calculator_part2(&contents)
    )
}
