pub fn part2_find_start_of_packet_marker(datastream: String) -> usize {
    parse(datastream.as_str(), 14)
}

pub fn part1_find_start_of_packet_marker(datastream: String) -> usize {
    parse(datastream.as_str(), 4)
}

fn parse(datastream: &str, number_of_uniques: usize) -> usize {
    let mut unique_chars: Vec<char> = vec![];
    let mut count = 0;

    for (index, char) in datastream.chars().enumerate() {
        if unique_chars.len() < number_of_uniques && !unique_chars.contains(&char) {
            unique_chars.push(char);
        } else {
            let mut after_double_found = unique_chars.split(|doubled| doubled == &char);

            after_double_found.next();

            unique_chars = after_double_found.next().unwrap().to_vec();
            unique_chars.push(char);
        }

        if unique_chars.len() == number_of_uniques {
            count = index + 1;
            break;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_datastream_correctly_with_four_unique_characters() {
        assert_eq!(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(parse("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(parse("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn parse_datastream_correctly_with_fourteen_unique_characters() {
        assert_eq!(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(parse("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(parse("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
