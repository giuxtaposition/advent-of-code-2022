pub fn part1_visible_trees(input: String) -> usize {
    let (grid, grid_height, grid_width) = to_grid(&input);

    visible_trees(grid, grid_width, grid_height)
}

fn visible_trees(grid: Vec<u32>, grid_width: usize, grid_height: usize) -> usize {
    grid.iter()
        .enumerate()
        .filter(|tree| is_visible(&grid, grid_width, grid_height, tree.0))
        .count()
}

fn to_grid(input: &str) -> (Vec<u32>, usize, usize) {
    let lines_vec = input.lines().collect::<Vec<&str>>();

    (
        lines_vec
            .iter()
            .flat_map(|row| row.chars().map(|char| char.to_digit(10).unwrap()))
            .collect(),
        lines_vec.len(),
        lines_vec.first().unwrap().chars().count(),
    )
}

fn is_visible(grid: &[u32], grid_width: usize, grid_height: usize, index: usize) -> bool {
    is_external_cell(grid_width, grid_height, index)
        || is_visible_in_row(grid, grid_width, index)
        || is_visible_in_column(grid, grid_width, grid_height, index)
}

fn is_visible_in_column(grid: &[u32], grid_width: usize, grid_height: usize, index: usize) -> bool {
    let mut current_cell = index - grid_width;
    let mut is_visible_top = true;

    while current_cell > 0 {
        if grid[index] <= grid[current_cell] {
            is_visible_top = false;
            break;
        }

        current_cell = current_cell.saturating_sub(grid_width)
    }

    if !is_visible_top {
        let mut is_visible_bottom = true;
        current_cell = index + grid_width;

        while current_cell < grid_width * grid_height {
            if grid[index] <= grid[current_cell] {
                is_visible_bottom = false;
                break;
            }

            current_cell += grid_width
        }

        is_visible_bottom
    } else {
        is_visible_top
    }
}

fn is_visible_in_row(grid: &[u32], grid_width: usize, index: usize) -> bool {
    let mut current_cell = index - 1;
    let mut is_visible_left = true;

    loop {
        if (current_cell + 1) % grid_width == 0 {
            break;
        }

        if grid[index] <= grid[current_cell] {
            is_visible_left = false;
            break;
        }

        current_cell = current_cell.saturating_sub(1)
    }

    if !is_visible_left {
        let mut is_visible_right = true;
        current_cell = index + 1;

        loop {
            if current_cell % grid_width == 0 {
                break;
            }

            if grid[index] <= grid[current_cell] {
                is_visible_right = false;
                break;
            }

            current_cell += 1;
        }
        is_visible_right
    } else {
        is_visible_left
    }
}

fn is_external_cell(grid_width: usize, grid_height: usize, index: usize) -> bool {
    let is_first_row = index < grid_width;
    let is_last_row = index > (grid_height * grid_width - grid_width);
    let is_first_column = (index + 1) % grid_width == 0;
    let is_last_column = index % grid_width == 0;

    is_first_row || is_last_row || is_first_column || is_last_column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_input_to_grid() {
        let input = "30373\n25512\n65332\n33549\n35390";

        assert_eq!(
            to_grid(input),
            (
                vec![3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0],
                5,
                5
            )
        )
    }

    #[test]
    fn find_visible_tree_based_on_surrounding_cells() {
        let trees_grid = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        assert_eq!(is_visible(&trees_grid, 5, 5, 6), true);
        assert_eq!(is_visible(&trees_grid, 5, 5, 7), true);
        assert_eq!(is_visible(&trees_grid, 5, 5, 8), false);
        assert_eq!(is_visible(&trees_grid, 5, 5, 11), true);
        assert_eq!(is_visible(&trees_grid, 5, 5, 12), false);
        assert_eq!(is_visible(&trees_grid, 5, 5, 17), true);
        assert_eq!(is_visible(&trees_grid, 5, 5, 0), true);
    }

    #[test]
    fn find_visible_tree_based_on_surrounding_and_outer_cells() {
        let trees_grid = vec![
            3, 0, 3, 7, 3, 2, 5, 1, 1, 2, 6, 1, 3, 1, 3, 3, 3, 1, 4, 9, 3, 5, 3, 9, 0,
        ];

        assert_eq!(is_visible(&trees_grid, 5, 5, 11), false);
        assert_eq!(is_visible(&trees_grid, 5, 5, 18), true);
    }

    #[test]
    fn find_visible_trees_from_outside_the_grid() {
        let trees_grid = vec![3, 0, 3, 7, 3, 2, 5, 5, 1];
        assert_eq!(visible_trees(trees_grid, 3, 3), 9);

        let trees_grid = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];
        assert_eq!(visible_trees(trees_grid, 5, 5), 21)
    }

    #[test]
    fn part1_example_test() {
        let input = "30373\n25512\n65332\n33549\n35390".to_string();

        assert_eq!(part1_visible_trees(input), 21)
    }
}
