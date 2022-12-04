use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!("{}", part_1_sum_of_overlap(contents));
}

fn part_1_sum_of_overlap(input: String) -> usize {
    input
        .lines()
        .map(|line| {
            let pairs: Vec<(i32, i32)> = line
                .split(",")
                .map(|pair| {
                    let mut split = pair.split("-").map(str::parse::<i32>);

                    (
                        split.next().unwrap().unwrap(),
                        split.next().unwrap().unwrap(),
                    )
                })
                .collect();

            one_of_contained_in(
                pairs.get(0).unwrap().to_owned(),
                pairs.get(1).unwrap().to_owned(),
            )
        })
        .filter(|value| *value == true)
        .count()
}

fn one_of_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    match (a, b) {
        (a, b) if a.0 >= b.0 && a.1 <= b.1 => true,
        (a, b) if b.0 >= a.0 && b.1 <= a.1 => true,
        (_, _) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_number_range_is_contained_in_other_or_viceversa() {
        assert_eq!(one_of_contained_in((2, 8), (3, 7)), true);
        assert_eq!(one_of_contained_in((6, 6), (4, 6)), true);
        assert_eq!(one_of_contained_in((2, 4), (6, 8)), false)
    }

    #[test]
    fn test_example_part_1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();

        assert_eq!(part_1_sum_of_overlap(input), 2)
    }
}
