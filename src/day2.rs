#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for Shape {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            &_ => panic!("Value not supported"),
        }
    }
}

enum RoundResult {
    Loss,
    Draw,
    Win,
}

impl From<&str> for RoundResult {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            &_ => panic!("Value not supported"),
        }
    }
}

pub fn part1_rounds_points_calculator(rounds: String) -> i32 {
    rounds
        .lines()
        .map(|round| {
            let shapes: Vec<Shape> = round.split(' ').map(Into::into).collect();
            round_points_calculator(*shapes.get(0).unwrap(), *shapes.get(1).unwrap())
        })
        .sum()
}

pub fn part2_rounds_points_calculator(rounds: String) -> i32 {
    rounds
        .lines()
        .map(|round| {
            let shapes: Vec<&str> = round.split(' ').collect();
            let opponent_hand = Shape::from(*shapes.first().unwrap());
            let player_hand = RoundResult::from(*shapes.get(1).unwrap());

            round_points_calculator(
                opponent_hand,
                from_round_result_to_player_hand(player_hand, opponent_hand),
            )
        })
        .sum()
}

fn from_round_result_to_player_hand(round_result: RoundResult, opponent_hand: Shape) -> Shape {
    match (round_result, opponent_hand) {
        (RoundResult::Loss, Shape::Rock)
        | (RoundResult::Draw, Shape::Scissors)
        | (RoundResult::Win, Shape::Paper) => Shape::Scissors,
        (RoundResult::Loss, Shape::Paper)
        | (RoundResult::Draw, Shape::Rock)
        | (RoundResult::Win, Shape::Scissors) => Shape::Rock,
        (RoundResult::Loss, Shape::Scissors)
        | (RoundResult::Draw, Shape::Paper)
        | (RoundResult::Win, Shape::Rock) => Shape::Paper,
    }
}
fn round_points_calculator(opponent_hand: Shape, player_hand: Shape) -> i32 {
    match (opponent_hand, player_hand) {
        (opponent_hand, player_hand) if opponent_hand == player_hand => player_hand as i32 + 3,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => player_hand as i32 + 6,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => player_hand as i32,
        (_, _) => panic!("This case should have been already covered"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_player_points_correctly() {
        let rounds = [
            [Shape::Rock, Shape::Rock],
            [Shape::Rock, Shape::Scissors],
            [Shape::Rock, Shape::Paper],
        ];
        let expected_points = [4, 3, 8];

        for (index, round) in rounds.iter().enumerate() {
            let expected_points = expected_points.get(index).unwrap().to_owned();

            let opponent_hand = round.get(0).unwrap();
            let player_hand = round.get(1).unwrap();

            assert_eq!(
                expected_points,
                round_points_calculator(*opponent_hand, *player_hand)
            );
        }
    }

    #[test]
    fn read_matches_correctly() {
        let lines = "A X\nA Y\nA Z\nB X\nB Y\nB Z\nC X\nC Y\nC Z\n".to_string();
        let expected_result = 45;
        assert_eq!(expected_result, part1_rounds_points_calculator(lines));
    }

    #[test]
    fn test_example_part_1() {
        let example = "A Y\nB X\nC Z".to_string();
        assert_eq!(part1_rounds_points_calculator(example), 15);
    }

    #[test]
    fn test_example_part_2() {
        let example = "A Y\nB X\nC Z".to_string();
        assert_eq!(part2_rounds_points_calculator(example), 12);
    }
}
