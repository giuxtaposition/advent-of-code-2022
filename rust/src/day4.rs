pub fn part1_sum_of_overlap(assignments: String) -> usize {
    sum_of_overlap(assignments, fully_contained_in)
}

pub fn part2_sum_of_overlap(assignments: String) -> usize {
    sum_of_overlap(assignments, partially_contained_in)
}

fn sum_of_overlap(
    input: String,
    overlap_function: impl Fn((i32, i32), (i32, i32)) -> bool,
) -> usize {
    input
        .lines()
        .filter(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let (elf1_range, elf2_range) = (get_range(elf1), get_range(elf2));

            overlap_function(elf1_range, elf2_range) || overlap_function(elf2_range, elf1_range)
        })
        .count()
}

fn get_range(input: &str) -> (i32, i32) {
    let (min, max) = input.split_once('-').unwrap();
    (min.parse().unwrap(), max.parse().unwrap())
}

fn fully_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn partially_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    (a.0 >= b.0 || a.1 >= b.0) && (a.0 <= b.0 || a.1 <= b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_number_range_is_contained_in_other() {
        assert_eq!(fully_contained_in((3, 7), (2, 8)), true);
        assert_eq!(fully_contained_in((6, 6), (4, 6)), true);
        assert_eq!(fully_contained_in((2, 4), (6, 8)), false)
    }

    #[test]
    fn test_example_part_1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();

        assert_eq!(sum_of_overlap(input, fully_contained_in), 2)
    }

    #[test]
    fn overlaps() {
        assert_eq!(partially_contained_in((5, 7), (7, 9)), true);
        assert_eq!(partially_contained_in((2, 8), (3, 7)), true);
        assert_eq!(partially_contained_in((4, 6), (6, 6)), true);
        assert_eq!(partially_contained_in((2, 6), (4, 8)), true);
        assert_eq!(partially_contained_in((2, 4), (6, 8)), false);
        assert_eq!(partially_contained_in((2, 3), (4, 5)), false);
    }
}
