pub fn part2_priorities_sum(input: String) -> i32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|group| {
            let strings = group.iter().map(ToString::to_string).collect();

            priority(common(strings) as u8)
        })
        .sum()
}

pub fn part1_priorities_sum(input: String) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let items = slice_in_half(line.to_string());

            priority(common(items) as u8)
        })
        .sum()
}

fn priority(item: u8) -> i32 {
    match item {
        b'a'..=b'z' => ((item - b'a') + 1) as i32,
        _ => ((item - b'A') + 1 + 26) as i32,
    }
}

fn slice_in_half(string: String) -> Vec<String> {
    if string.len() % 2 != 0 {
        panic!("Cannot split in half")
    }

    let split = string.split_at(string.len() / 2);
    vec![split.0.to_string(), split.1.to_string()]
}

fn common(mut strings: Vec<String>) -> char {
    strings
        .pop()
        .unwrap()
        .chars()
        .find(|&a| strings.iter().all(|string| string.contains(a)))
        .unwrap_or_else(|| panic!("Could not find common char"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_common_char_between_two_str() {
        let a = "vJrwpWtwJgWr".to_string();
        let b = "hcsFMMfFFhFp".to_string();

        assert_eq!(common(vec![a, b]), 'p')
    }

    #[test]
    fn slice_string_exactly_in_half() {
        let string = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let expected_result = vec![
            "jqHRNqRjqzjGDLGL".to_string(),
            "rsFMfFZSrLrFZsSL".to_string(),
        ];

        assert_eq!(slice_in_half(string), expected_result)
    }

    #[test]
    fn find_priority_from_items() {
        let items = vec!['p', 'L', 'P', 'v', 't', 's'];
        let expected_priorities = vec![16, 38, 42, 22, 20, 19];

        for (index, item) in items.iter().enumerate() {
            assert_eq!(
                priority(item.to_owned() as u8),
                expected_priorities.get(index).unwrap().to_owned()
            )
        }
    }

    #[test]
    fn test_example_part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();

        assert_eq!(part1_priorities_sum(input), 157)
    }

    #[test]
    fn find_common_char_between_three_str() {
        let a = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let b = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let c = "PmmdzqPrVvPwwTWBwg".to_string();

        assert_eq!(common(vec![a, b, c]), 'r')
    }

    #[test]
    fn test_example_part_2() {
        let input ="vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();

        assert_eq!(part2_priorities_sum(input), 70);
    }
}
