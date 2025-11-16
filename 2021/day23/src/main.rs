use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn energy(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn target_room(&self) -> usize {
        match self {
            Amphipod::A => 0,
            Amphipod::B => 1,
            Amphipod::C => 2,
            Amphipod::D => 3,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; N]; 4],
}

fn parse_input<const N: usize>(input: &str) -> State<N> {
    let lines: Vec<&str> = input.lines().collect();

    // Dynamically detect room positions by finding uppercase letters in the first room row
    let first_room_line = lines[2].chars().collect::<Vec<_>>();
    let room_positions: Vec<usize> = first_room_line
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| {
            if c.is_ascii_uppercase() && matches!(c, 'A' | 'B' | 'C' | 'D') {
                Some(i)
            } else {
                None
            }
        })
        .collect();

    let mut rooms = [[None; N]; 4];

    // Parse N rows directly from input
    for row_idx in 0..N {
        let row = lines[2 + row_idx].chars().collect::<Vec<_>>();
        for (i, &pos) in room_positions.iter().enumerate() {
            if i < 4 {
                rooms[i][row_idx] = Amphipod::from_char(row[pos]);
            }
        }
    }

    State {
        hallway: [None; 11],
        rooms,
    }
}

impl<const N: usize> State<N> {
    fn is_complete(&self) -> bool {
        for (i, room) in self.rooms.iter().enumerate() {
            let target = match i {
                0 => Amphipod::A,
                1 => Amphipod::B,
                2 => Amphipod::C,
                3 => Amphipod::D,
                _ => unreachable!(),
            };
            if !room.iter().all(|&a| a == Some(target)) {
                return false;
            }
        }
        true
    }

    fn room_entrance(&self, room: usize) -> usize {
        2 + room * 2
    }

    fn is_room_ready(&self, room: usize, amphipod: Amphipod) -> bool {
        let target = amphipod.target_room();
        if room != target {
            return false;
        }
        self.rooms[room]
            .iter()
            .all(|&a| a.is_none() || a == Some(amphipod))
    }

    fn can_move_between(&self, from: usize, to: usize) -> bool {
        if from == to {
            return true;
        }
        let (start, end) = if from < to {
            (from + 1, to)
        } else {
            (to, from - 1)
        };
        self.hallway[start..=end].iter().all(|&pos| pos.is_none())
    }

    fn get_moves(&self) -> Vec<(State<N>, usize)> {
        let mut moves = Vec::new();

        // Move from rooms to hallway
        for room_idx in 0..4 {
            // Find the topmost amphipod in this room
            let mut found_depth = None;
            for depth in 0..N {
                if self.rooms[room_idx][depth].is_some() {
                    found_depth = Some(depth);
                    break;
                }
            }

            if let Some(depth) = found_depth {
                let amphipod = self.rooms[room_idx][depth].unwrap();

                // Check if amphipod should stay (already in correct room with correct amphipods below)
                let mut should_stay = room_idx == amphipod.target_room();
                if should_stay {
                    // Check all positions below are correct
                    for d in (depth + 1)..N {
                        if self.rooms[room_idx][d] != Some(amphipod) {
                            should_stay = false;
                            break;
                        }
                    }
                }

                if should_stay {
                    continue;
                }

                let entrance = self.room_entrance(room_idx);

                // Try all hallway positions
                for hall_pos in 0..11 {
                    // Can't stop in front of rooms
                    if hall_pos == 2 || hall_pos == 4 || hall_pos == 6 || hall_pos == 8 {
                        continue;
                    }

                    if self.hallway[hall_pos].is_none() && self.can_move_between(entrance, hall_pos)
                    {
                        let mut new_state = self.clone();
                        new_state.rooms[room_idx][depth] = None;
                        new_state.hallway[hall_pos] = Some(amphipod);

                        let distance = hall_pos.abs_diff(entrance) + depth + 1;
                        let cost = distance * amphipod.energy();
                        moves.push((new_state, cost));
                    }
                }
            }
        }

        // Move from hallway to rooms
        for hall_pos in 0..11 {
            if let Some(amphipod) = self.hallway[hall_pos] {
                let target_room = amphipod.target_room();

                if self.is_room_ready(target_room, amphipod) {
                    let entrance = self.room_entrance(target_room);

                    if self.can_move_between(hall_pos, entrance) {
                        // Find deepest available spot (go as deep as possible)
                        let mut depth = None;
                        for d in (0..N).rev() {
                            if self.rooms[target_room][d].is_none() {
                                depth = Some(d);
                                break;
                            }
                        }

                        if let Some(depth) = depth {
                            let mut new_state = self.clone();
                            new_state.hallway[hall_pos] = None;
                            new_state.rooms[target_room][depth] = Some(amphipod);

                            let distance = hall_pos.abs_diff(entrance) + depth + 1;
                            let cost = distance * amphipod.energy();
                            moves.push((new_state, cost));
                        }
                    }
                }
            }
        }

        moves
    }
}

fn solve_p1(input: &str) -> usize {
    let initial_state = parse_input::<2>(input);

    let result = dijkstra(
        &initial_state,
        |state| state.get_moves(),
        |state| state.is_complete(),
    );

    result.map(|(_, cost)| cost).unwrap_or(0)
}

fn solve_p2(input: &str) -> usize {
    let initial_state = parse_input::<4>(input);

    let result = dijkstra(
        &initial_state,
        |state| state.get_moves(),
        |state| state.is_complete(),
    );

    result.map(|(_, cost)| cost).unwrap_or(0)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let start = std::time::Instant::now();
    let answer = solve_p1(&input);
    let elapsed = start.elapsed();
    println!("Part 1: {answer}, elapsed: {elapsed:.1?}");

    let input_p2 = std::fs::read_to_string("input_p2.txt").unwrap();
    let start = std::time::Instant::now();
    let answer = solve_p2(&input_p2);
    let elapsed = start.elapsed();
    println!("Part 2: {answer}, elapsed: {elapsed:.1?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 12521);
        let input = std::fs::read_to_string("test_input1.txt").unwrap();
        let answer = solve_p2(&input);
        assert_eq!(answer, 44169);
    }
}
