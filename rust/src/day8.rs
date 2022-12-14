pub fn part1_visible_trees(input: String) -> usize {
    visible_trees(&to_grid(&input))
}

pub fn part2_highest_scenic_score(input: String) -> u32 {
    highest_scenic_score(&to_grid(&input))
}

fn highest_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
    grid.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .map(|(y, _)| scenic_score(grid, x, y))
                .collect::<Vec<u32>>()
        })
        .max()
        .unwrap()
}

fn visible_trees(grid: &Vec<Vec<u32>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter(|(y, _)| is_visible(grid, x, *y))
                .count()
        })
        .sum()
}

fn scenic_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return 0;
    }

    let mut left = 0;
    for cell in grid[x][..y].iter().rev() {
        left += 1;

        println!("cell {}", cell);

        if grid[x][y] <= *cell {
            break;
        }
    }

    let mut right = 0;
    for cell in grid[x][y + 1..].iter() {
        right += 1;

        if grid[x][y] <= *cell {
            break;
        }
    }

    let mut top = 0;
    for row in grid[..x].iter().rev() {
        top += 1;

        if grid[x][y] <= row[y] {
            break;
        }
    }

    let mut bottom = 0;
    for row in grid[(x + 1)..].iter() {
        bottom += 1;
        if grid[x][y] <= row[y] {
            break;
        }
    }

    top * left * right * bottom
}

fn is_visible(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || x == grid.len() - 1 || y == grid[x].len() - 1 {
        return true;
    }

    let left = *grid[x][..y].iter().max().unwrap() < grid[x][y];
    let right = *grid[x][y + 1..].iter().max().unwrap() < grid[x][y];
    let top = grid[..x].iter().map(|row| row[y]).max().unwrap() < grid[x][y];
    let bottom = grid[x + 1..].iter().map(|row| row[y]).max().unwrap() < grid[x][y];

    left || right || top || bottom
}

fn to_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input_to_grid() {
        let input = "30373\n25512\n65332\n33549\n35390";

        assert_eq!(
            to_grid(input),
            (vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0]
            ])
        )
    }

    #[test]
    fn find_visible_tree_based_on_surrounding_cells() {
        let trees_grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(is_visible(&trees_grid, 1, 1), true);
        assert_eq!(is_visible(&trees_grid, 1, 2), true);
        assert_eq!(is_visible(&trees_grid, 1, 3), false);
        assert_eq!(is_visible(&trees_grid, 2, 2), false);
        assert_eq!(is_visible(&trees_grid, 2, 3), true);
        assert_eq!(is_visible(&trees_grid, 4, 0), true);
        assert_eq!(is_visible(&trees_grid, 0, 0), true);
    }

    #[test]
    fn find_visible_tree_based_on_surrounding_and_outer_cells() {
        let trees_grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 1, 1, 2],
            vec![6, 1, 3, 1, 3],
            vec![3, 3, 1, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(is_visible(&trees_grid, 2, 2), false);
        assert_eq!(is_visible(&trees_grid, 4, 1), true);
    }

    #[test]
    fn find_visible_trees_from_outside_the_grid() {
        let trees_grid = vec![vec![3, 0, 3], vec![7, 3, 2], vec![5, 5, 1]];
        assert_eq!(visible_trees(&trees_grid), 9);

        let trees_grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(visible_trees(&trees_grid), 21)
    }

    #[test]
    fn part1_example_test() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();

        assert_eq!(part1_visible_trees(input), 21)
    }

    #[test]
    fn single_tree_scenic_score() {
        let grid = to_grid("30373\n25512\n65332\n33549\n35390");

        assert_eq!(scenic_score(&grid, 1, 2), 4);

        let grid = to_grid("30373\n25512\n65332\n33549\n35390");

        assert_eq!(scenic_score(&grid, 3, 2), 8)
    }
}
