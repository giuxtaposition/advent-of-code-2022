use std::str::FromStr;

pub fn part1_top_crates_calculator(input: String) -> String {
    top_crates_calculator(input, false)
}

pub fn part2_top_crates_calculator(input: String) -> String {
    top_crates_calculator(input, true)
}

fn top_crates_calculator(input: String, keep_order: bool) -> String {
    let (stacks_schema, instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks_schema(stacks_schema);

    for instruction in instructions.lines() {
        stacks = move_crate(instruction, stacks, keep_order);
    }

    top_crates(stacks)
}

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        let container: Vec<usize> = instruction
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Ok(Self {
            amount: container[0],
            from: container[1] - 1,
            to: container[2] - 1,
        })
    }
}

fn top_crates(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect::<String>()
}

fn parse_stacks_schema(stacks_schema: &str) -> Vec<Vec<char>> {
    let (stacks_str, stacks_numbers) = stacks_schema.rsplit_once('\n').unwrap();
    let stack_size = stacks_numbers
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let mut result: Vec<Vec<char>> = vec![Vec::new(); stack_size];

    stacks_str.lines().map(normalize).rev().for_each(|line| {
        line.chars().enumerate().for_each(|(index, char)| {
            if char.is_alphabetic() {
                result[index].push(char.to_owned())
            }
        });
    });

    result
}

fn move_crate(
    instruction_string: &str,
    mut stacks: Vec<Vec<char>>,
    keep_order: bool,
) -> Vec<Vec<char>> {
    let instruction = instruction_string.parse::<Instruction>().unwrap();

    let n_cargo_to_keep = stacks[instruction.from].len() - instruction.amount;

    let mut stack_to_update = stacks[instruction.from].split_off(n_cargo_to_keep);

    if !keep_order {
        stack_to_update.reverse()
    }

    stack_to_update.iter().for_each(|cargo_to_move| {
        stacks[instruction.to].push(*cargo_to_move);
    });

    stacks
}

fn normalize(input: &str) -> String {
    input
        .replace(|c: char| !c.is_whitespace() && !c.is_alphanumeric(), "")
        .replace("  ", " ")
        .chars()
        .step_by(2)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_whitespace() {
        assert_eq!(normalize("[N] [C] [N]"), "NCN");
        assert_eq!(normalize("[N] [C]   "), "NC ");
        assert_eq!(normalize("    [D]    "), " D ");
    }

    #[test]
    fn parse_schema_correctly() {
        let input = "    [D]    \n[N] [C]   \n[Z] [M] [P]\n1   2   3";
        let expected_result: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(parse_stacks_schema(input), expected_result)
    }

    #[test]
    fn move_crates_to_correct_stack() {
        assert_eq!(
            move_crate(
                "move 1 from 2 to 1",
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                false
            ),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );

        assert_eq!(
            move_crate(
                "move 3 from 1 to 3",
                vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
                false
            ),
            vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']]
        )
    }

    #[test]
    fn top_crate_from_each_stack() {
        assert_eq!(
            top_crates(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]),
            "NDP"
        )
    }

    #[test]
    fn move_crates_to_correct_stack_keeping_order() {
        assert_eq!(
            move_crate(
                "move 1 from 2 to 1",
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
                true
            ),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );

        assert_eq!(
            move_crate(
                "move 3 from 1 to 3",
                vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']],
                true
            ),
            vec![vec![], vec!['M', 'C'], vec!['P', 'Z', 'N', 'D']]
        )
    }
}
