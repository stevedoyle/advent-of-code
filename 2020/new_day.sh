#!/bin/bash

# Script to create placeholder files for a new Advent of Code day
# Usage: ./new_day.sh XX (where XX is the day number, e.g., 07)

if [ -z "$1" ]; then
    echo "Usage: $0 <day_number>"
    echo "Example: $0 07"
    exit 1
fi

DAY=$1

# Validate day format (should be 01-25)
if ! [[ "$DAY" =~ ^[0-2][0-9]$ ]]; then
    echo "Error: Day should be in format XX (01-25)"
    exit 1
fi

# Get the directory where the script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# File paths
TEMPLATE_FILE="src/bin/day_template.rs"
NEW_BIN_FILE="src/bin/day${DAY}.rs"
INPUT_FILE="inputs/day${DAY}.txt"
TEST_INPUT_FILE="inputs/day${DAY}_test.txt"
CARGO_FILE="Cargo.toml"

# Check if files already exist
if [ -f "$NEW_BIN_FILE" ]; then
    echo "Error: $NEW_BIN_FILE already exists"
    exit 1
fi

# Create the new binary file from template
echo "Creating $NEW_BIN_FILE..."
sed "s/DAY_NUMBER/$DAY/g; s/EXPECTED_P1/0/g; s/EXPECTED_P2/0/g" "$TEMPLATE_FILE" > "$NEW_BIN_FILE"

# Create empty input files
echo "Creating $INPUT_FILE..."
touch "$INPUT_FILE"

echo "Creating $TEST_INPUT_FILE..."
touch "$TEST_INPUT_FILE"

# Update Cargo.toml - add new binary entry
echo "Updating $CARGO_FILE..."
echo "" >> "$CARGO_FILE"
echo "[[bin]]" >> "$CARGO_FILE"
echo "name = \"day${DAY}\"" >> "$CARGO_FILE"
echo "path = \"src/bin/day${DAY}.rs\"" >> "$CARGO_FILE"

echo "✓ Successfully created files for day ${DAY}!"
echo ""
echo "Files created:"
echo "  - $NEW_BIN_FILE"
echo "  - $INPUT_FILE"
echo "  - $TEST_INPUT_FILE"
echo ""
echo "Cargo.toml updated with new binary entry"
echo ""
echo "Run with: cargo run --bin day${DAY}"
