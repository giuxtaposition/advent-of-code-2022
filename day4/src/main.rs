use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!(
        "{}",
        sum_of_overlap(contents.clone(), one_of_fully_contained_in)
    );
    println!(
        "{}",
        sum_of_overlap(contents, one_of_partially_contained_in)
    );
}

fn sum_of_overlap(
    input: String,
    overlap_function: impl Fn((i32, i32), (i32, i32)) -> bool,
) -> usize {
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

            overlap_function(
                pairs.get(0).unwrap().to_owned(),
                pairs.get(1).unwrap().to_owned(),
            )
        })
        .filter(|value| *value == true)
        .count()
}

fn one_of_partially_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    match (a, b) {
        (a, b) if !partially_contained_in(a, b) || !partially_contained_in(b, a) => false,
        (_, _) => true,
    }
}

fn one_of_fully_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    match (a, b) {
        (a, b) if fully_contained_in(a, b) || fully_contained_in(b, a) => true,
        (_, _) => false,
    }
}

fn fully_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 >= b.0 && a.1 <= b.1
}

fn partially_contained_in(a: (i32, i32), b: (i32, i32)) -> bool {
    a.0 >= b.0 || a.1 >= b.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_if_number_range_is_contained_in_other_or_viceversa() {
        assert_eq!(one_of_fully_contained_in((2, 8), (3, 7)), true);
        assert_eq!(one_of_fully_contained_in((6, 6), (4, 6)), true);
        assert_eq!(one_of_fully_contained_in((2, 4), (6, 8)), false)
    }

    #[test]
    fn test_example_part_1() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8".to_string();

        assert_eq!(sum_of_overlap(input, one_of_fully_contained_in), 2)
    }

    #[test]
    fn overlaps() {
        assert_eq!(one_of_partially_contained_in((5, 7), (7, 9)), true);
        assert_eq!(one_of_partially_contained_in((2, 8), (3, 7)), true);
        assert_eq!(one_of_partially_contained_in((6, 6), (4, 6)), true);
        assert_eq!(one_of_partially_contained_in((2, 6), (4, 8)), true);
        assert_eq!(one_of_partially_contained_in((2, 4), (6, 8)), false);
        assert_eq!(one_of_partially_contained_in((2, 3), (4, 5)), false);
    }
}
