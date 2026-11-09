#!/usr/bin/env bash
# Scaffold a day and fetch its input.
#
#   scripts/new-day.sh <day> [--no-fetch]
#
# The input is downloaded with your AoC session cookie, taken from $AOC_SESSION
# or ~/.config/aoc/session. Existing files are never overwritten.
set -euo pipefail

YEAR=2015
cd "$(dirname "$0")/.."

day=$((10#${1:?usage: scripts/new-day.sh <day> [--no-fetch]}))
dd=$(printf '%02d' "$day")

bin="src/bin/day${dd}.rs"
if [[ -e $bin ]]; then
  echo "$bin already exists, leaving it alone"
else
  sed "s/{{DAY}}/${day}/" templates/day.rs > "$bin"
  echo "created $bin"
fi

touch "inputs/day${dd}_test.txt"

input="inputs/day${dd}.txt"
if [[ ${2:-} == --no-fetch || -s $input ]]; then
  exit 0
fi

session="${AOC_SESSION:-$(cat ~/.config/aoc/session 2>/dev/null || true)}"
if [[ -z $session ]]; then
  echo "no session cookie (set AOC_SESSION or ~/.config/aoc/session); skipping input download"
  exit 0
fi

curl -fsS \
  -A "github.com/stevedoyle/advent-of-code new-day.sh" \
  -b "session=${session}" \
  "https://adventofcode.com/${YEAR}/day/${day}/input" \
  -o "$input"
echo "downloaded $input"
