# Advent of Code 2021 - Rust Solutions

This is a Rust workspace containing solutions for Advent of Code 2021.

## Structure

Each day's solution is organized in its own crate:

- `day01/` - Day 1 solution
- `day02/` - Day 2 solution (when created)
- etc.

## Building and Running

To build all solutions:

```bash
cargo build
```

To run a specific day's solution:

```bash
cargo run -p day01
```

To run tests for all days:

```bash
cargo test
```

To run tests for a specific day:

```bash
cargo test -p day01
```

## Adding New Days

To add a new day:

1. Create a new directory (e.g., `day02/`)
2. Add a `Cargo.toml` file following the pattern in `day01/`
3. Add the new crate to the `members` list in the workspace `Cargo.toml`
4. Create your solution in `src/main.rs`

## Common Dependencies

The workspace provides common dependencies that are useful for Advent of Code:

- `regex` - Regular expressions
- `itertools` - Additional iterator methods
- `anyhow` - Error handling

To use these in a day's solution, add them to that day's `Cargo.toml`:

```toml
[dependencies]
regex.workspace = true
itertools.workspace = true
anyhow.workspace = true
```
