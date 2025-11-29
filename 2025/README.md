# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) in Rust.

## Structure

This year uses a single Cargo project with multiple binaries for better compilation speed and code sharing:

- `src/lib.rs` - Shared utility functions (input reading, parsing, etc.)
- `src/bin/` - Individual day solutions (day01.rs through day25.rs)
- `inputs/` - Input files (dayXX.txt and dayXX_test.txt)

## Usage

Run a specific day:
```bash
cargo run --bin day01
cargo run --bin day02
# etc...
```

Run with release optimizations (for performance measurement):
```bash
cargo run --release --bin day01
```

Run tests for a specific day:
```bash
cargo test --bin day01
```

Run all tests:
```bash
cargo test
```

Build everything:
```bash
cargo build
cargo build --release
```

## Adding a New Day

1. Edit the corresponding file in `src/bin/dayXX.rs`
2. Add your input to `inputs/dayXX.txt`
3. Add test input to `inputs/dayXX_test.txt`
4. Update the test expectations in the `#[test]` section
5. Run and verify: `cargo test --bin dayXX && cargo run --bin dayXX`

## Shared Utilities

The `src/lib.rs` file provides common functions:
- `read_input(day)` - Read the main input file
- `read_test_input(day)` - Read the test input file
- `parse_lines()` - Parse lines into a vector
- `parse_grid()` - Parse a character grid
- `parse_digit_grid()` - Parse a digit grid

Feel free to add more shared utilities as needed!
