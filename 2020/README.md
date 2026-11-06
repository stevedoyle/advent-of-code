# Advent of Code 2020

Solutions for [Advent of Code 2020](https://adventofcode.com/2020) in Rust (edition 2024, needs Rust 1.85+).

## Layout

- `src/lib.rs` — input reading, `run()` with timing, parsing helpers (`parse_lines`, `nums`)
- `src/point.rs` — `Point { x, y }` with `+ - * neg`, directions, turning
- `src/grid.rs` — flat `Grid<T>` indexed by `Point`
- `src/bin/dayNN.rs` — one binary per day (auto-discovered by Cargo)
- `templates/day.rs` — starting point for each day
- `inputs/` — `dayNN.txt` (gitignored) and `dayNN_test.txt` / `dayNN_testN.txt` (committed)

Coordinates are `x` = column, `y` = row, with `y` increasing downwards.

## New day

```bash
export AOC_SESSION=...        # or put it in ~/.config/aoc/session
scripts/new-day.sh 1          # creates src/bin/day01.rs and inputs, downloads input
```

Then paste the example into `inputs/day01_test.txt` and fill in the expected answers.

## Running

```bash
cargo run --release --bin day01
cargo test --bin day01
cargo test                    # everything, including lib helpers
```

Paths are resolved from the crate root, so commands work from any directory.

For shortest paths, BFS, flood fill etc., add the `pathfinding` crate (see `Cargo.toml`) rather than hand-rolling them.
