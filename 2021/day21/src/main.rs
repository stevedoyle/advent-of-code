fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .filter_map(|line| {
            line.split(':')
                .nth(1)
                .and_then(|num_str| num_str.trim().parse::<usize>().ok())
        })
        .collect()
}

fn solve_p1(input: &str) -> usize {
    let starting_positions = parse_input(input);
    let num_players = starting_positions.len();
    let mut positions = starting_positions.clone();
    let mut scores = vec![0; num_players];
    let mut die_value = 1;
    let mut total_rolls = 0;
    let winning_score = 1000;

    loop {
        for player in 0..num_players {
            let roll_sum: usize = (0..3)
                .map(|_| {
                    let value = die_value;
                    die_value = if die_value == 100 { 1 } else { die_value + 1 };
                    total_rolls += 1;
                    value
                })
                .sum();

            positions[player] = (positions[player] - 1 + roll_sum) % 10 + 1;
            scores[player] += positions[player];

            if scores[player] >= winning_score {
                let losing_score: usize = scores
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != player)
                    .map(|(_, &score)| score)
                    .sum();
                return losing_score * total_rolls;
            }
        }
    }
}

fn solve_p2(input: &str) -> usize {
    use std::collections::HashMap;

    type GameState = (usize, usize, usize, usize, usize);
    type WinCounts = (usize, usize);
    type Cache = HashMap<GameState, WinCounts>;

    let starting_positions = parse_input(input);

    // Frequencies of sums when rolling 3-sided die 3 times
    // Sum: (3,4,5,6,7,8,9) appears with frequency: (1,3,6,7,6,3,1)
    let roll_frequencies = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    // Memoization: (pos1, score1, pos2, score2, current_player) -> (wins1, wins2)
    let mut cache: Cache = HashMap::new();

    fn count_wins(
        state: GameState,
        roll_frequencies: &[(usize, usize)],
        cache: &mut Cache,
    ) -> WinCounts {
        let (pos1, score1, pos2, score2, current_player) = state;

        // Check if game is already won
        if score1 >= 21 {
            return (1, 0);
        }
        if score2 >= 21 {
            return (0, 1);
        }

        // Check cache
        if let Some(&result) = cache.get(&state) {
            return result;
        }

        let mut total_wins = (0, 0);

        for &(roll_sum, frequency) in roll_frequencies {
            let new_state = if current_player == 0 {
                let new_pos = (pos1 - 1 + roll_sum) % 10 + 1;
                let new_score = score1 + new_pos;
                (new_pos, new_score, pos2, score2, 1 - current_player)
            } else {
                let new_pos = (pos2 - 1 + roll_sum) % 10 + 1;
                let new_score = score2 + new_pos;
                (pos1, score1, new_pos, new_score, 1 - current_player)
            };

            let (wins1, wins2) = count_wins(new_state, roll_frequencies, cache);

            total_wins.0 += wins1 * frequency;
            total_wins.1 += wins2 * frequency;
        }

        cache.insert(state, total_wins);
        total_wins
    }

    let (wins1, wins2) = count_wins(
        (starting_positions[0], 0, starting_positions[1], 0, 0),
        &roll_frequencies,
        &mut cache,
    );

    wins1.max(wins2)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let start = std::time::Instant::now();
    let answer = solve_p2(&input);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_solve_with_test_input() {
        let input = TEST_INPUT;
        let answer = solve_p1(&input);
        assert_eq!(answer, 739785);
        let answer = solve_p2(&input);
        assert_eq!(answer, 444356092776315);
    }
}
