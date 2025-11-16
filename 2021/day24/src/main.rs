fn parse_input(input: &str) -> Vec<(i64, i64, i64)> {
    // The MONAD program consists of 14 blocks, each processing one digit
    // Each block has the structure with 3 key parameters:
    // - div_z: divides z (either 1 or 26)
    // - add_x: added to x after mod
    // - add_y: added to y (and then to z)

    let lines: Vec<&str> = input.lines().collect();
    let mut params = Vec::new();

    for chunk in lines.chunks(18) {
        if chunk.len() < 18 {
            break;
        }

        // Extract the 3 key parameters from each block
        let div_z: i64 = chunk[4].split_whitespace().nth(2).unwrap().parse().unwrap();
        let add_x: i64 = chunk[5].split_whitespace().nth(2).unwrap().parse().unwrap();
        let add_y: i64 = chunk[15]
            .split_whitespace()
            .nth(2)
            .unwrap()
            .parse()
            .unwrap();

        params.push((div_z, add_x, add_y));
    }

    params
}

fn find_model_number(params: &[(i64, i64, i64)], largest: bool) -> i64 {
    // The MONAD program implements a stack-like operation
    // When div_z = 1, we push (digit + add_y) onto stack
    // When div_z = 26, we pop and check constraint: popped + add_x == current digit

    let mut stack = Vec::new();
    let mut constraints = Vec::new();

    for (i, &(div_z, add_x, add_y)) in params.iter().enumerate() {
        if div_z == 1 {
            // Push operation
            stack.push((i, add_y));
        } else {
            // Pop operation - creates a constraint between two digits
            let (j, prev_add_y) = stack.pop().unwrap();
            // Constraint: digit[j] + prev_add_y + add_x == digit[i]
            // Or: digit[i] = digit[j] + prev_add_y + add_x
            constraints.push((j, i, prev_add_y + add_x));
        }
    }

    // Initialize with the target values based on largest/smallest
    let mut digits = if largest { vec![9; 14] } else { vec![1; 14] };

    // Apply constraints
    for (j, i, diff) in constraints {
        if largest {
            // For largest: maximize both digits
            if diff >= 0 {
                // digit[i] = digit[j] + diff
                // To maximize, set digit[j] = 9 - diff (if positive diff)
                digits[j] = (9 - diff).max(1);
                digits[i] = digits[j] + diff;
            } else {
                // digit[j] = digit[i] - diff
                digits[i] = (9 + diff).max(1);
                digits[j] = digits[i] - diff;
            }
        } else {
            // For smallest: minimize both digits
            if diff >= 0 {
                // digit[i] = digit[j] + diff
                digits[j] = 1;
                digits[i] = (digits[j] + diff).min(9);
            } else {
                // digit[j] = digit[i] - diff
                digits[i] = 1;
                digits[j] = (digits[i] - diff).min(9);
            }
        }
    }

    // Convert digits to number
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}

fn solve_p1(input: &str) -> i64 {
    let params = parse_input(input);
    find_model_number(&params, true)
}

fn solve_p2(input: &str) -> i64 {
    let params = parse_input(input);
    find_model_number(&params, false)
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
    #[test]
    fn test_solve_with_test_input() {
        // Day 24 doesn't have a test input - the solution is specific to the input
        // This test is a placeholder
    }
}
