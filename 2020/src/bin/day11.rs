use aoc2020::*;

fn step_seatmap(
    seatmap: &[Vec<char>],
    occupied_threshold: usize,
    use_visibility: bool,
) -> (Vec<Vec<char>>, bool) {
    let mut new_seatmap = seatmap.to_vec();
    let mut changed = false;

    for pos in grid::positions(seatmap) {
        let seat = seatmap[pos.0][pos.1];
        if seat == '.' {
            continue;
        }

        let occupied_count = if use_visibility {
            count_visible_occupied(seatmap, pos)
        } else {
            count_adjacent_occupied(seatmap, pos)
        };

        if seat == 'L' && occupied_count == 0 {
            new_seatmap[pos.0][pos.1] = '#';
            changed = true;
        } else if seat == '#' && occupied_count >= occupied_threshold {
            new_seatmap[pos.0][pos.1] = 'L';
            changed = true;
        }
    }

    (new_seatmap, changed)
}

fn count_adjacent_occupied(seatmap: &[Vec<char>], pos: (usize, usize)) -> usize {
    let (row, col) = pos;
    grid::valid_neighbors8(seatmap, row as isize, col as isize)
        .iter()
        .map(|(r, c)| Some(&seatmap[*r][*c]))
        .filter(|&c| c == Some(&'#'))
        .count()
}

fn count_visible_occupied(seatmap: &[Vec<char>], pos: (usize, usize)) -> usize {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let (row, col) = pos;
    let rows = seatmap.len() as isize;
    let cols = seatmap[0].len() as isize;

    directions
        .iter()
        .filter(|(dr, dc)| {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;

            while r >= 0 && r < rows && c >= 0 && c < cols {
                let seat = seatmap[r as usize][c as usize];
                if seat == '#' {
                    return true;
                } else if seat == 'L' {
                    return false;
                }
                // Continue looking if it's floor ('.')
                r += dr;
                c += dc;
            }
            false
        })
        .count()
}

fn count_occupied_seats(seatmap: &[Vec<char>]) -> usize {
    seatmap
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn solve_p1(input: &str) -> usize {
    let mut seatmap = parse_grid(input);
    loop {
        let (new_seatmap, changed) = step_seatmap(&seatmap, 4, false);
        if !changed {
            return count_occupied_seats(&new_seatmap);
        }
        seatmap = new_seatmap;
    }
}

fn solve_p2(input: &str) -> usize {
    let mut seatmap = parse_grid(input);
    loop {
        let (new_seatmap, changed) = step_seatmap(&seatmap, 5, true);
        if !changed {
            return count_occupied_seats(&new_seatmap);
        }
        seatmap = new_seatmap;
    }
}

fn main() {
    let input = read_input(11);

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

    #[test]
    fn test_solve_with_test_input() {
        let input = read_test_input(11);
        let answer = solve_p1(&input);
        assert_eq!(answer, 37);
        let answer = solve_p2(&input);
        assert_eq!(answer, 26);
    }
}
