use aoc2020::*;

struct Field {
    name: String,
    ranges: Vec<(usize, usize)>,
}

impl From<&str> for Field {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split(": ").collect();
        let name = parts[0].to_string();
        let ranges: Vec<(usize, usize)> = parts[1]
            .split(" or ")
            .map(|r| {
                let bounds: Vec<usize> = r.split('-').map(|n| n.parse().unwrap()).collect();
                (bounds[0], bounds[1])
            })
            .collect();
        Field { name, ranges }
    }
}

fn parse_input(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let fields: Vec<Field> = sections[0].lines().map(Field::from).collect();

    let your_ticket: Vec<usize> = sections[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<usize>> = sections[2]
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    (fields, your_ticket, nearby_tickets)
}

fn solve_p1(input: &str) -> usize {
    let (fields, _your_ticket, nearby_tickets) = parse_input(input);

    let mut error_rate = 0;
    for ticket in nearby_tickets {
        for &value in &ticket {
            if !fields.iter().any(|field| {
                field
                    .ranges
                    .iter()
                    .any(|&(min, max)| value >= min && value <= max)
            }) {
                error_rate += value;
            }
        }
    }
    error_rate
}

fn solve_p2(input: &str) -> usize {
    let (fields, your_ticket, nearby_tickets) = parse_input(input);

    let valid_tickets: Vec<&Vec<usize>> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|&value| {
                fields.iter().any(|field| {
                    field
                        .ranges
                        .iter()
                        .any(|&(min, max)| value >= min && value <= max)
                })
            })
        })
        .collect();

    let num_fields = fields.len();
    let mut possible_fields: Vec<Vec<usize>> = vec![vec![]; num_fields];
    for col in 0..num_fields {
        'field_loop: for (field_idx, field) in fields.iter().enumerate() {
            for ticket in &valid_tickets {
                let value = ticket[col];
                if !field
                    .ranges
                    .iter()
                    .any(|&(min, max)| value >= min && value <= max)
                {
                    continue 'field_loop;
                }
            }
            possible_fields[col].push(field_idx);
        }
    }

    let mut determined_fields: Vec<Option<usize>> = vec![None; num_fields];
    while determined_fields.iter().any(|f| f.is_none()) {
        for col in 0..num_fields {
            if determined_fields[col].is_none() && possible_fields[col].len() == 1 {
                let field_idx = possible_fields[col][0];
                determined_fields[col] = Some(field_idx);
                for (other_col, possible_field) in
                    possible_fields.iter_mut().enumerate().take(num_fields)
                {
                    if other_col != col {
                        possible_field.retain(|&idx| idx != field_idx);
                    }
                }
            }
        }
    }
    let mut product = 1;
    for (col, &field_idx_opt) in determined_fields.iter().enumerate() {
        if let Some(field_idx) = field_idx_opt {
            if fields[field_idx].name.starts_with("departure") {
                product *= your_ticket[col];
            }
        }
    }
    product
}

fn main() {
    let input = read_input(16);

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
        let input = read_test_input(16);
        let answer = solve_p1(&input);
        assert_eq!(answer, 71);
    }
}
