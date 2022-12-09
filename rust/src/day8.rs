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
    let lines = input.lines();
    let lines_vec = lines.clone().collect::<Vec<&str>>();

    (
        lines
            .flat_map(|row| row.chars().map(|char| char.to_digit(10).unwrap()))
            .collect(),
        lines_vec.len(),
        lines_vec.first().unwrap().chars().count(),
    )
}

fn is_visible(grid: &[u32], grid_width: usize, grid_height: usize, index: usize) -> bool {
    if is_external_cell(grid_width, grid_height, index) {
        return true;
    }

    if is_visible_in_row(grid, grid_width, index) {
        return true;
    }

    if is_visible_in_column(grid, grid_width, grid_height, index) {
        return true;
    }

    false
}

fn is_visible_in_column(grid: &[u32], grid_width: usize, grid_height: usize, index: usize) -> bool {
    let mut current_cell = index - grid_width;
    let mut is_visible_top = true;
    let mut is_visible_bottom = true;

    while current_cell > 0 {
        if grid[index] <= grid[current_cell] {
            is_visible_top = false;
            break;
        }

        current_cell = current_cell.saturating_sub(grid_width)
    }

    current_cell = index + grid_width;

    while current_cell < grid_width * grid_height {
        if grid[index] <= grid[current_cell] {
            is_visible_bottom = false;
            break;
        }

        current_cell += grid_width
    }

    is_visible_top || is_visible_bottom
}

fn is_visible_in_row(grid: &[u32], grid_width: usize, index: usize) -> bool {
    let mut current_cell = index - 1;
    let mut is_visible_left = true;
    let mut is_visible_right = true;

    loop {
        if (current_cell + 1) % grid_width == 0 {
            break;
        }

        if grid[index] <= grid[current_cell] {
            is_visible_left = false;
            break;
        }

        if let Some(next_current_cell) = current_cell.checked_sub(1) {
            current_cell = next_current_cell
        } else {
            break;
        }
    }

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

    is_visible_left || is_visible_right
}

fn is_external_cell(grid_width: usize, grid_height: usize, index: usize) -> bool {
    index < grid_width
        || index > (grid_height * grid_width - grid_width)
        || index % grid_width == 0
        || (index + 1) % grid_width == 0
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
