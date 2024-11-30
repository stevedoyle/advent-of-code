use std::collections::HashMap;

// Blocks in the starting position. Lowest 7 bits are used to represent the block.
const BLOCKS: [&[u8]; 5] = [
    // Horizontal row
    &[0b0011110],
    // Cross
    &[0b0001000, 0b0011100, 0b0001000],
    // J
    &[0b0000100, 0b0000100, 0b0011100],
    // Vertical column
    &[0b0010000, 0b0010000, 0b0010000, 0b0010000],
    // Square
    &[0b0011000, 0b0011000],
];

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

#[allow(dead_code)]
fn display_grid(grid: &[u8]) {
    let row_count = grid.len() - 1;
    grid.iter()
        .rev()
        .enumerate()
        .for_each(|(i, row)| println!("{:02}: {:07b}", row_count - i, row));
}

#[allow(dead_code)]
fn display_block(block: &[u8]) {
    block.iter().for_each(|row| println!(">   {:07b}", row));
}

fn solve(input: &str, num_blocks: usize) -> usize {
    let jets = parse_input(input);
    let mut grid: Vec<u8> = vec![0; 3];

    let mut block_iter = BLOCKS.iter().cycle();
    let mut jet_iter = jets.iter().cycle();

    let mut jet_count = 0;
    let mut seen: HashMap<(usize, usize, u8), (usize, usize)> = HashMap::new();

    (0..num_blocks).for_each(|block_idx| {
        let block = block_iter.next().unwrap().to_vec();
        let empty_rows_at_top = grid.iter().rev().take_while(|&&row| row == 0).count();
        let rows_to_add_or_remove = empty_rows_at_top as i32 - block.len() as i32 - 3;
        if rows_to_add_or_remove < 0 {
            grid.extend(vec![0; -rows_to_add_or_remove as usize]);
        } else if rows_to_add_or_remove > 0 {
            grid.truncate(grid.len() - rows_to_add_or_remove as usize);
        }
        jet_count += drop_block(&mut jet_iter, &mut grid, block);

        let amount = grid.iter().filter(|&&row| row != 0).count();

        let key = (
            block_idx % BLOCKS.len(),
            jet_count % jets.len(),
            grid[grid.len() - 4],
        );
        if seen.contains_key(&key) {
            let (prev_block_idx, prev_amount) = seen[&key];
            let cycle_length = block_idx - prev_block_idx;
            let remaining_blocks = num_blocks - block_idx;
            let cycles = remaining_blocks / cycle_length;
            let remaining = remaining_blocks % cycle_length;
            let amount_per_cycle = amount - prev_amount;
            let total_amount = prev_amount + cycles * amount_per_cycle;
            println!("Cycle length: {}", cycle_length);
            // return total_amount;
        } else {
            seen.insert(key, (block_idx, amount));
            // display_grid(&grid);
        }
    });

    grid.iter().filter(|&&row| row != 0).count()
}

fn solve_p1(input: &str) -> usize {
    let num_blocks = 2022;
    let jets = parse_input(input);
    let mut grid: Vec<u8> = vec![0; 3];

    let mut block_iter = BLOCKS.iter().cycle();
    let mut jet_iter = jets.iter().cycle();

    (0..num_blocks).for_each(|_| {
        let block = block_iter.next().unwrap().to_vec();
        let empty_rows_at_top = grid.iter().rev().take_while(|&&row| row == 0).count();
        let rows_to_add_or_remove = empty_rows_at_top as i32 - block.len() as i32 - 3;
        if rows_to_add_or_remove < 0 {
            grid.extend(vec![0; -rows_to_add_or_remove as usize]);
        } else if rows_to_add_or_remove > 0 {
            grid.truncate(grid.len() - rows_to_add_or_remove as usize);
        }
        drop_block(&mut jet_iter, &mut grid, block);
    });

    grid.iter().filter(|&&row| row != 0).count()
}

fn solve_p2(input: &str) -> usize {
    // TODO: Complete the cycle detection
    // solve(input, 1_000_000_000_000)
    0
}

fn drop_block<'a>(
    jet_iter: &mut impl Iterator<Item = &'a Jet>,
    grid: &mut Vec<u8>,
    block: Vec<u8>,
) -> usize {
    let mut block = block;
    let mut prev_block = block.clone();
    let mut grid_row = grid.len() - 1;
    let mut jet_count = 0;

    loop {
        // Move based on jet stream
        move_block(&mut block, *jet_iter.next().unwrap());
        if !is_valid(&grid, grid_row, &block) {
            block = prev_block.clone();
        } else {
            prev_block = block.clone();
        }
        jet_count += 1;
        // display_block(&block);

        if grid_row == 0 {
            merge_block(grid, &block, grid_row);
            return jet_count;
        }

        // Move down
        grid_row -= 1;
        if !is_valid(&grid, grid_row, &block) {
            // display_block(&block);
            // display_grid(&grid);
            merge_block(grid, &block, grid_row + 1);
            return jet_count;
        }
    }
}

fn merge_block(grid: &mut Vec<u8>, block: &[u8], grid_idx: usize) {
    // println!("Merging block at index: {}", grid_idx);
    let block_height = block.len();
    (0..block_height).for_each(|i| {
        grid[grid_idx - i] |= block[i];
    });
}

fn is_valid(grid: &[u8], grid_idx: usize, block: &[u8]) -> bool {
    // println!("Checking if block is valid at index: {}", grid_idx);
    let block_height = block.len();

    if block_height > grid_idx + 1 {
        return false;
    }

    let mut skip_size = grid_idx as i32 + 1 - block_height as i32;
    if skip_size < 0 {
        skip_size = 0;
    }
    let grid_rows: Vec<u8> = grid
        .iter()
        .skip(skip_size as usize)
        .take(block_height)
        .cloned()
        .collect();
    let block_rows: Vec<&u8> = block.iter().rev().collect();
    grid_rows
        .iter()
        .zip(block_rows.iter())
        .all(|(&grid_row, &&block_row)| (grid_row & block_row) == 0)
}

fn move_block(block: &mut [u8], jet: Jet) {
    // println!("Moving block: {:?}", jet);
    match jet {
        Jet::Left => shift_left(block),
        Jet::Right => shift_right(block),
    }
}

fn shift_left(block: &mut [u8]) {
    if can_shift_left(block) {
        for row in block.iter_mut() {
            *row <<= 1;
        }
    }
}

fn can_shift_left(block: &[u8]) -> bool {
    block.iter().all(|&row| row & 0b100_0000 == 0)
}

fn shift_right(block: &mut [u8]) {
    if can_shift_right(block) {
        for row in block.iter_mut() {
            *row >>= 1;
        }
    }
}

fn can_shift_right(block: &[u8]) -> bool {
    block.iter().all(|&row| row & 0b000_0001 == 0)
}

fn main() {
    let input = include_str!("../input.txt");
    let answer = solve_p1(input);
    println!("Part 1: {answer}");
    let answer = solve_p2(input);
    println!("Part 2: {answer}");
}

fn parse_input(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '>' => Jet::Right,
            '<' => Jet::Left,
            _ => panic!("Invalid character in input"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_solve_with_test_input() {
        let answer = solve_p1(INPUT);
        assert_eq!(answer, 3068);
        let answer = solve_p2(INPUT);
        assert_eq!(answer, 1514285714288);
    }
}
