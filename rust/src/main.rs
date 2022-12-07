use advent_of_code_2022::*;

fn main() {
    println!(
        r"
        🎄 day1 part1: {:?} 🎄
        🎄 day1 part2: {:?} 🎄
        🎄 day2 part1: {:?} 🎄
        🎄 day2 part2: {:?} 🎄
        🎄 day3 part1: {:?} 🎄
        🎄 day3 part2: {:?} 🎄
        🎄 day4 part1: {:?} 🎄
        🎄 day4 part2: {:?} 🎄
        🎄 day5 part1: {:?} 🎄
        🎄 day5 part2: {:?} 🎄
        🎄 day6 part1: {:?} 🎄
        🎄 day6 part2: {:?} 🎄
        🎄 day7 part1: {:?} 🎄
        🎄 day7 part2: {:?} 🎄
        ",
        day1::part1_top_calories(utils::read_input("day1/input.txt")),
        day1::part2_top_3_calories(utils::read_input("day1/input.txt")),
        day2::part1_rounds_points_calculator(utils::read_input("day2/input.txt")),
        day2::part2_rounds_points_calculator(utils::read_input("day2/input.txt")),
        day3::part1_priorities_sum(utils::read_input("day3/input.txt")),
        day3::part2_priorities_sum(utils::read_input("day3/input.txt")),
        day4::part1_sum_of_overlap(utils::read_input("day4/input.txt")),
        day4::part2_sum_of_overlap(utils::read_input("day4/input.txt")),
        day5::part1_top_crates_calculator(utils::read_input("day5/input.txt")),
        day5::part2_top_crates_calculator(utils::read_input("day5/input.txt")),
        day6::part1_find_start_of_packet_marker(utils::read_input("day6/input.txt")),
        day6::part2_find_start_of_packet_marker(utils::read_input("day6/input.txt")),
        day7::part1_sum_of_sizes_with_directories_less_than_1000(utils::read_input(
            "day7/input.txt"
        )),
        day7::part2_total_size_of_directory_to_delete(utils::read_input("day7/input.txt"))
    );
}
