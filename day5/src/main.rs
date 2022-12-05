use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    println!("{}", part_1_top_crates(contents))
}

fn part_1_top_crates(contents: String) -> String {
    let lines = contents.lines().collect::<Vec<&str>>();

    let position = lines.iter().position(|line| line.is_empty());

    let (schema, instructions) = lines.split_at(position.unwrap());

    let mut parsed_schema = parse_schema(schema.to_vec());

    for instruction in instructions {
        if !instruction.is_empty() {
            parsed_schema = move_crate(instruction, parsed_schema);
        }
    }

    top_crates(parsed_schema)
}

fn top_crates<'a>(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| {
            let mut chars = stack.clone();
            chars.pop().unwrap()
        })
        .collect::<String>()
}

fn parse_schema(mut input: Vec<&str>) -> Vec<Vec<char>> {
    let stack_size = input
        .pop()
        .unwrap()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap()
        .try_into()
        .unwrap();
    let stacks_schema: Vec<String> = input.iter().map(|line| normalize(line)).collect();

    let mut result: Vec<Vec<char>> = vec![Vec::new(); stack_size];

    stacks_schema.iter().rev().for_each(|line| {
        line.chars().enumerate().for_each(|(index, char)| {
            if char.is_alphabetic() {
                result[index].push(char.to_owned())
            }
        });
    });

    result
}

fn move_crate(instructions_string: &str, mut stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let instructions = parse_instruction(instructions_string);

    let n_cargo_to_keep = stacks[instructions[1] - 1].len() - instructions[0];

    stacks[instructions[1] - 1]
        .split_off(n_cargo_to_keep)
        .iter()
        .rev()
        .for_each(|cargo_to_move| {
            stacks[instructions[2] - 1].push(*cargo_to_move);
        });

    stacks
}

fn parse_instruction(instruction: &str) -> Vec<usize> {
    instruction
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

fn normalize<'a>(input: &'a str) -> String {
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
        let input = vec!["    [D]    ", "[N] [C]   ", "[Z] [M] [P]", "1   2   3"];
        let expected_result: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(parse_schema(input), expected_result)
    }

    #[test]
    fn move_crates_to_correct_stack() {
        assert_eq!(
            move_crate(
                "move 1 from 2 to 1",
                vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
            ),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );

        assert_eq!(
            move_crate(
                "move 3 from 1 to 3",
                vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
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
}
