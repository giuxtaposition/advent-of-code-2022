use std::{collections::HashMap, path::PathBuf};

pub fn part2_total_size_of_directory_to_delete(terminal_output: String) -> u32 {
    let sizes = sum_of_directories_sizes(terminal_output);
    let total_disk_space = 70000000;
    let needed = 30000000;
    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = total_disk_space - root;

    *sizes
        .values()
        .filter(|size| available + *size >= needed)
        .min()
        .unwrap()
}

pub fn part1_sum_of_sizes_with_directories_less_than_1000(terminal_output: String) -> u32 {
    sum_of_directories_sizes(terminal_output)
        .values()
        .filter(|size| *size <= &100000)
        .sum()
}

fn sum_of_directories_sizes(terminal_output: String) -> HashMap<PathBuf, u32> {
    let mut sizes = HashMap::new();
    let mut directories_affected = Vec::new();

    for line in terminal_output.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let line_parts: Vec<_> = line.split_whitespace().collect();

        match line_parts[..] {
            ["$", "cd", ".."] => {
                directories_affected.pop();
            }
            ["$", "cd", name] => directories_affected.push(name),
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for index in 0..directories_affected.len() {
                    let path = PathBuf::from_iter(&directories_affected[..=index]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_sizes_of_directories_with_sum_less_than_100000() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k".to_string();

        assert_eq!(
            part1_sum_of_sizes_with_directories_less_than_1000(input),
            95437
        )
    }

    #[test]
    fn total_sizes_of_directory_to_delete_to_have_enough_space() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k".to_string();

        assert_eq!(part2_total_size_of_directory_to_delete(input), 24933642)
    }
}
