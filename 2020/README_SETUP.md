# Day Setup Script

## Usage

```bash
python setupday.py <day_number>
```

## Example

```bash
python setupday.py 2
```

This will create:
- `src/bin/day02.rs` - Rust solution file from template
- `inputs/day02_test.txt` - Empty test input file
- `inputs/day02.txt` - Empty puzzle input file

## Features

- ✅ Validates day number (1-25)
- ✅ Replaces template placeholders automatically
- ✅ Protects against accidental overwrite
- ✅ Creates empty input files
- ✅ Provides next steps instructions

## Template Placeholders

The script replaces the following placeholders in `src/bin/day_template.rs`:
- `DAY_NUMBER` → actual day number (e.g., 2)
- `EXPECTED_P1` → 0 (update with your test expectation)
- `EXPECTED_P2` → 0 (update with your test expectation)

## Next Steps After Setup

1. Add test input to `inputs/dayXX_test.txt`
2. Add puzzle input to `inputs/dayXX.txt`
3. Update test expectations in the generated file
4. Implement your solution!

Run with:
```bash
cargo test --bin day02  # Run tests
cargo run --bin day02   # Run solution
```
