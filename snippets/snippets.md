# Collection of code snippets that have been useful to me in the past.

## Using Matrix from the pathfinding crate

### Reading a map from a string

```rust
use pathfinding::matrix::Matrix;

fn main() {
    let map = "#########\n\
               #S......#\n\
               #.######.#\n\
               #.#....#.#\n\
               #.###.#..#\n\
               #...#.##.#\n\
               #.##....G#\n\
               #########";

    let matrix = Matrix::from_rows(map.lines().map(|line| line.chars().collect::<Vec<_>>()))?;
    println!("{:?}", matrix);

    // Row and column counts are available as fields.
    assert_eq!(matrix.rows, 8);
    assert_eq!(matrix.cols, 9);
    // Access elements with the (row, col) index.
    assert_eq!(matrix[(1, 1)], 'S');

    // Bounds checking
    assert_eq!(matrix.get((0, 0)), Some(&'#'));
    assert_eq!(matrix.within_bounds((0, 0)), true);

    // Iterating over the rows in the matrix
    for row in matrix.iter() {
    }

    // Iterating over the rows in the matrix
    for col in matrix.column_iter() {
    }

    // Iterating over the values in the matrix
    for (idx, value) in matrix.values().enumerate() {
        println!("[{}, {}]: {}", idx / matrix.columns, idx % matrix.columns, value);
    }

    // Iterate over the matrix indices, first row first.
    for (row, col) in matrix.keys() {
    }

    // Print the matrix
    matrix.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{}", cell));
        println!();
    });

    // Get the neighbors of a cell
    let neighbors = matrix.neighbors(&(1, 1));
    neighbors.for_each(|(row, col)| {
        println!("Neighbor: ({}, {})", row, col);
    });
}
```

### Finding the shortest path using Dijkstra's algorithm

```rust
use pathfinding::prelude::dijkstra;

fn main() {
    let map = "#########\n\
               #S......#\n\
               #.######.#\n\
               #.#....#.#\n\
               #.###.#..#\n\
               #...#.##.#\n\
               #.##....G#\n\
               #########";

    let matrix = Matrix::from_rows(map.lines().map(|line| line.chars().collect::<Vec<_>>()))?;
    let start = matrix.values().find_position('S').unwrap();
    let goal = matrix.values().find_position('G').unwrap();

    let result = dijkstra(
        &start,
        |p| matrix.neighbors(p),
        |p| p == &goal,
    );

    match result {
        Some(path) => {
            println!("Shortest path: {:?}", path);
            let mut matrix = matrix.clone();
            for (row, col) in path {
                matrix[(row, col)] = '.';
            }
            matrix.iter().for_each(|row| {
                row.iter().for_each(|cell| print!("{}", cell));
                println!();
            });
        }
        None => {
            println!("No path found");
        }
    }
}
```
