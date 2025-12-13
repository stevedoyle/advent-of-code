#!/usr/bin/env python3
"""
Setup script for Advent of Code 2020 days.
Usage: python setupday.py <day_number>
Example: python setupday.py 2
"""

import sys
import os
from pathlib import Path


def setup_day(day_num: int):
    """Set up files for a specific AoC day."""
    # Validate day number
    if not 1 <= day_num <= 25:
        print(f"Error: Day number must be between 1 and 25, got {day_num}")
        sys.exit(1)

    # Format day number with leading zero
    day_str = f"{day_num:02d}"

    # Define paths
    script_dir = Path(__file__).parent
    src_bin_dir = script_dir / "src" / "bin"
    inputs_dir = script_dir / "inputs"
    template_file = src_bin_dir / "day_template.rs"
    target_file = src_bin_dir / f"day{day_str}.rs"
    test_input_file = inputs_dir / f"day{day_str}_test.txt"
    input_file = inputs_dir / f"day{day_str}.txt"

    # Check if template exists
    if not template_file.exists():
        print(f"Error: Template file not found at {template_file}")
        sys.exit(1)

    # Check if day file already exists
    if target_file.exists():
        response = input(f"Warning: {target_file} already exists. Overwrite? (y/N): ")
        if response.lower() != 'y':
            print("Aborted.")
            sys.exit(0)

    # Read template
    with open(template_file, 'r') as f:
        template_content = f.read()

    # Replace placeholders
    day_content = template_content.replace("DAY_NUMBER", str(day_num))
    day_content = day_content.replace("EXPECTED_P1", "0")
    day_content = day_content.replace("EXPECTED_P2", "0")

    # Create target file
    with open(target_file, 'w') as f:
        f.write(day_content)
    print(f"✓ Created {target_file}")

    # Create empty input files if they don't exist
    for input_path in [test_input_file, input_file]:
        if not input_path.exists():
            input_path.touch()
            print(f"✓ Created {input_path}")
        else:
            print(f"  Skipped {input_path} (already exists)")

    print(f"\n✅ Day {day_num} setup complete!")
    print(f"\nNext steps:")
    print(f"1. Add test input to: {test_input_file}")
    print(f"2. Add puzzle input to: {input_file}")
    print(f"3. Update test expectations in: {target_file}")
    print(f"4. Implement solution in: {target_file}")
    print(f"\nRun tests with: cargo test --bin day{day_str}")
    print(f"Run solution with: cargo run --bin day{day_str}")


def main():
    if len(sys.argv) != 2:
        print("Usage: python setupday.py <day_number>")
        print("Example: python setupday.py 2")
        sys.exit(1)

    try:
        day_num = int(sys.argv[1])
    except ValueError:
        print(f"Error: Invalid day number '{sys.argv[1]}'. Must be an integer.")
        sys.exit(1)

    setup_day(day_num)


if __name__ == "__main__":
    main()
