pub fn part1_find_start_of_packet_marker(datastream: String) -> usize {
    parse(datastream.as_str())
}

fn parse(datastream: &str) -> usize {
    let mut unique_chars: Vec<char> = vec![];
    let mut count = 0;

    for (index, char) in datastream.chars().enumerate() {
        if unique_chars.len() < 4 && !unique_chars.contains(&char) {
            unique_chars.push(char);
        } else {
            let mut after_double_found = unique_chars.split(|doubled| doubled == &char);

            after_double_found.next();

            unique_chars = after_double_found.next().unwrap().to_vec();
            unique_chars.push(char);
        }

        if unique_chars.len() == 4 {
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
    fn parse_datastream_correctly() {
        assert_eq!(parse("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(parse("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(parse("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(parse("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(parse("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
