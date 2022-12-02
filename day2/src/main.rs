use day2::rounds_points_calculator;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!(
        "Player points after following strategy guide:{:?}",
        rounds_points_calculator(contents)
    );
}
