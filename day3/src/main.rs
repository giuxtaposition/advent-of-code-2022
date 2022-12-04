use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!("{}", part_1_priorities_sum(contents))
}

fn part_1_priorities_sum(input: String) -> i32 {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let items = slice_in_half(line.to_string());
            let common = common(items);
            return priority(&common);
        })
        .sum()
}

fn priority(item: &str) -> i32 {
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .split("")
        .position(|char| char == item)
        .unwrap()
        .try_into()
        .unwrap()
}

fn slice_in_half(string: String) -> (String, String) {
    if string.len() % 2 != 0 {
        panic!("Cannot split in half")
    }

    let split = string.split_at(string.len() / 2);
    (split.0.to_string(), split.1.to_string())
}

fn common((a, b): (String, String)) -> String {
    a.chars()
        .find(|&a| b.contains(a))
        .unwrap_or_else(|| panic!("Could not find common char"))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_common_char_between_two_str() {
        let a = "vJrwpWtwJgWr".to_string();
        let b = "hcsFMMfFFhFp".to_string();
        let expected_char = "p";

        assert_eq!(common((a, b)), expected_char)
    }

    #[test]
    fn slice_string_exactly_in_half() {
        let string = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let expected_result = (
            "jqHRNqRjqzjGDLGL".to_string(),
            "rsFMfFZSrLrFZsSL".to_string(),
        );

        assert_eq!(slice_in_half(string), expected_result)
    }

    #[test]
    fn find_priority_from_items() {
        let items = vec!["p", "L", "P", "v", "t", "s"];
        let expected_priorities = vec![16, 38, 42, 22, 20, 19];

        for (index, item) in items.iter().enumerate() {
            assert_eq!(
                priority(item),
                expected_priorities.get(index).unwrap().to_owned()
            )
        }
    }

    #[test]
    fn test_example_part_1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".to_string();
        let expected_sum = 157;

        assert_eq!(part_1_priorities_sum(input), expected_sum)
    }
}
