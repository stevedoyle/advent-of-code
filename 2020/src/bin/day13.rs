use aoc2020::*;

const DAY: u8 = 13;

fn parse_input(input: &str) -> (usize, Vec<&str>) {
    let earliest_departure_time = input.lines().next().unwrap().parse().unwrap();
    let buses = input.lines().nth(1).unwrap().split(',').collect();
    (earliest_departure_time, buses)
}

fn solve_p1(input: &str) -> usize {
    let (edt, buses) = parse_input(input);
    let min_next_bus = buses
        .iter()
        .filter_map(|&bus| {
            if bus == "x" {
                None
            } else {
                let bus_id: usize = bus.parse().unwrap();
                let wait_time = bus_id - (edt % bus_id);
                Some((bus_id, wait_time))
            }
        })
        .min_by_key(|&(_, wait_time)| wait_time)
        .unwrap();
    min_next_bus.0 * min_next_bus.1
}

fn solve_p2(input: &str) -> usize {
    let (_, buses) = parse_input(input);
    let mut t = 0;

    let mut step = 1;
    for (i, &bus) in buses.iter().enumerate() {
        if bus == "x" {
            continue;
        }
        let bus_id: usize = bus.parse().unwrap();
        while (t + i) % bus_id != 0 {
            t += step;
        }
        step *= bus_id;
    }

    t
}

fn main() {
    run(DAY, solve_p1, solve_p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_p1(&read_test_input(DAY, 1)), 295);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_p2(&read_test_input(DAY, 1)), 1068781); // For buses: 7,13,x,x,59,x,31,19
    }
}
