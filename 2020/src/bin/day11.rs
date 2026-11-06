use aoc2020::*;

const DAY: u8 = 11;

fn step_seatmap(
    seatmap: &Grid<u8>,
    occupied_threshold: usize,
    use_visibility: bool,
) -> (Grid<u8>, bool) {
    let mut new_seatmap = seatmap.clone();
    let mut changed = false;

    for pos in seatmap.points() {
        let seat = seatmap[pos];
        if seat == b'.' {
            continue;
        }

        let occupied_count = if use_visibility {
            count_visible_occupied(seatmap, pos)
        } else {
            count_adjacent_occupied(seatmap, pos)
        };

        if seat == b'L' && occupied_count == 0 {
            new_seatmap[pos] = b'#';
            changed = true;
        } else if seat == b'#' && occupied_count >= occupied_threshold {
            new_seatmap[pos] = b'L';
            changed = true;
        }
    }

    (new_seatmap, changed)
}

fn count_adjacent_occupied(seatmap: &Grid<u8>, pos: Point) -> usize {
    seatmap
        .neighbors8(pos)
        .filter(|&p| seatmap[p] == b'#')
        .count()
}

fn count_visible_occupied(seatmap: &Grid<u8>, pos: Point) -> usize {
    DIRS8
        .iter()
        .filter(|&&dir| {
            let mut p = pos + dir;

            while let Some(&seat) = seatmap.get(p) {
                if seat == b'#' {
                    return true;
                } else if seat == b'L' {
                    return false;
                }
                // Continue looking if it's floor ('.')
                p += dir;
            }
            false
        })
        .count()
}

fn count_occupied_seats(seatmap: &Grid<u8>) -> usize {
    seatmap.cells.iter().filter(|&&c| c == b'#').count()
}

fn solve_p1(input: &str) -> usize {
    let mut seatmap = Grid::parse(input);
    loop {
        let (new_seatmap, changed) = step_seatmap(&seatmap, 4, false);
        if !changed {
            return count_occupied_seats(&new_seatmap);
        }
        seatmap = new_seatmap;
    }
}

fn solve_p2(input: &str) -> usize {
    let mut seatmap = Grid::parse(input);
    loop {
        let (new_seatmap, changed) = step_seatmap(&seatmap, 5, true);
        if !changed {
            return count_occupied_seats(&new_seatmap);
        }
        seatmap = new_seatmap;
    }
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 37);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 26);
    }
}
