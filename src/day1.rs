pub fn part1_top_calories(calories_list: String) -> i32 {
    sum_of_greatest_numbers(calories_calculator(calories_list), 1)
}

pub fn part2_top_3_calories(calories_list: String) -> i32 {
    sum_of_greatest_numbers(calories_calculator(calories_list), 3)
}

fn calories_calculator(calories_list: String) -> Vec<i32> {
    let mut sum: Vec<i32> = vec![0];
    let mut index: usize = 0;

    for line in calories_list.lines() {
        match line.is_empty() {
            true => {
                index = index + 1;
                sum.push(0)
            }
            false => {
                let calorie = line.parse::<i32>().unwrap();

                sum[index] = sum[index] + calorie;
            }
        }
    }
    sum
}

fn sum_of_greatest_numbers(mut numbers: Vec<i32>, number_of_results: usize) -> i32 {
    numbers.sort_by(|a, b| b.partial_cmp(a).unwrap());

    numbers.split_at(number_of_results).0.to_vec().iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_first_reindeer_calories() {
        let first_reindeer = "7914\n5032\n11424\n2567\n4123\n3567\n7346".to_string();

        let expected_sum: i32 = 7914 + 5032 + 11424 + 2567 + 4123 + 3567 + 7346;

        assert_eq!(
            &expected_sum,
            calories_calculator(first_reindeer).first().unwrap()
        );
    }

    #[test]
    fn get_second_reindeer_calories() {
        let first_reindeer_and_second_reindeer = "7914\n5032\n11424\n2567\n4123\n3567\n7346\n\n1334\n2173\n5437\n1104\n1872\n1148\n6547\n3149\n5923\n5705\n4036\n5348\n1100".to_string();

        let expected_sum: i32 = 1334
            + 2173
            + 5437
            + 1104
            + 1872
            + 1148
            + 6547
            + 3149
            + 5923
            + 5705
            + 4036
            + 5348
            + 1100;

        assert_eq!(
            &expected_sum,
            calories_calculator(first_reindeer_and_second_reindeer)
                .get(1)
                .unwrap()
        );
    }

    #[test]
    fn get_greatest_number() {
        let numbers: Vec<i32> = vec![154, 23, 200];

        assert_eq!(200, sum_of_greatest_numbers(numbers, 1));
    }

    #[test]
    fn get_3_greatest_numbers() {
        let numbers: Vec<i32> = vec![154, 23, 200, 500, 1];
        let expected_sum = 500 + 200 + 154;

        assert_eq!(expected_sum, sum_of_greatest_numbers(numbers, 3));
    }
}
