use std::env;
use std::fs;

use advent_of_code_2022::calories_calculator;
use advent_of_code_2022::sum_of_greatest_numbers;

fn main() {
    let current_dir = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let file_path = format!("{}/input.txt", current_dir);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let all_reindeers_calories = calories_calculator(contents);

    println!(
        "The reindeer with the greatest calories has: {:?}",
        sum_of_greatest_numbers(all_reindeers_calories.clone(), 1)
    );

    println!(
        "The top 3 reindeers with the greatest calories has: {:?}",
        sum_of_greatest_numbers(all_reindeers_calories, 3)
    )
}
