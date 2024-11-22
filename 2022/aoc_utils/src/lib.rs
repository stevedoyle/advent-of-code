use grid::Grid;

pub fn parse_grid_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new(0, 0);

    for line in input.lines() {
        grid.push_row(line.trim().chars().collect());
    }
    grid
}

pub fn grid_neighbors(grid: &Grid<char>, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            let neigh_row = row as i32 + i;
            let neigh_col = col as i32 + j;
            if neigh_row < 0 || neigh_row >= grid.rows() as i32 {
                continue;
            }
            if neigh_col < 0 || neigh_col >= grid.cols() as i32 {
                continue;
            }
            neighbors.push((neigh_row as usize, neigh_col as usize));
        }
    }
    neighbors
}

pub fn grid_neighbors_without_diagonals(
    grid: &Grid<char>,
    (row, col): (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }

            if i != 0 && j != 0 {
                continue;
            }

            let neigh_row = row as i32 + i;
            let neigh_col = col as i32 + j;
            if neigh_row < 0 || neigh_row >= grid.rows() as i32 {
                continue;
            }
            if neigh_col < 0 || neigh_col >= grid.cols() as i32 {
                continue;
            }
            neighbors.push((neigh_row as usize, neigh_col as usize));
        }
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_input() {
        let gstr = "..#\n#.#\n.#.";
        let grid = parse_grid_input(gstr);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
        assert_eq!(grid[(0, 0)], '.');
        assert_eq!(grid[(0, 1)], '.');
        assert_eq!(grid[(0, 2)], '#');
        assert_eq!(grid[(1, 0)], '#');
        assert_eq!(grid[(1, 1)], '.');
        assert_eq!(grid[(1, 2)], '#');
        assert_eq!(grid[(2, 0)], '.');
        assert_eq!(grid[(2, 1)], '#');
        assert_eq!(grid[(2, 2)], '.');
    }

    #[test]
    fn test_parse_grid_input_empty() {
        let gstr = "";
        let grid = parse_grid_input(gstr);
        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.cols(), 0);
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid::new(3, 3);
        let neighbors = grid_neighbors(&grid, (1, 1));
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&(0, 0)));
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(0, 2)));
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 2)));
        assert!(neighbors.contains(&(2, 0)));
        assert!(neighbors.contains(&(2, 1)));
        assert!(neighbors.contains(&(2, 2)));
    }

    #[test]
    fn test_neighbors_corner() {
        let grid = Grid::new(3, 3);
        let neighbors = grid_neighbors(&grid, (0, 0));
        assert_eq!(neighbors.len(), 3);
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 1)));
    }

    #[test]
    fn test_neighbors_edge() {
        let grid = Grid::new(3, 3);
        let neighbors = grid_neighbors(&grid, (1, 0));
        assert_eq!(neighbors.len(), 5);
        assert!(neighbors.contains(&(0, 0)));
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 1)));
        assert!(neighbors.contains(&(2, 0)));
        assert!(neighbors.contains(&(2, 1)));
    }

    #[test]
    fn test_neighbors_bottom_edge() {
        let grid = Grid::new(3, 3);
        let neighbors = grid_neighbors(&grid, (2, 1));
        assert_eq!(neighbors.len(), 5);
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 1)));
        assert!(neighbors.contains(&(1, 2)));
        assert!(neighbors.contains(&(2, 0)));
        assert!(neighbors.contains(&(2, 2)));
    }

    #[test]
    fn test_neighbors_without_diagonals() {
        let grid = Grid::new(3, 3);
        let neighbors = grid_neighbors_without_diagonals(&grid, (1, 1));
        assert_eq!(neighbors.len(), 4);
        assert!(neighbors.contains(&(0, 1)));
        assert!(neighbors.contains(&(1, 0)));
        assert!(neighbors.contains(&(1, 2)));
        assert!(neighbors.contains(&(2, 1)));
    }
}
