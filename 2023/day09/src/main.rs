use std::{error::Error, fs};

fn compute_diff(data: &[i64]) -> Vec<i64> {
    data.iter()
        .zip(data.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>()
}

fn get_layers(data: &[i64]) -> Vec<Vec<i64>> {
    let mut layers: Vec<Vec<i64>> = Vec::with_capacity(100);
    layers.push(Vec::from(data));
    loop {
        let diff = layers.last().unwrap();
        let next_diff = compute_diff(diff);
        let done = next_diff.iter().all(|x| *x == 0);
        layers.push(next_diff);
        if done {
            break;
        }
    }
    layers
}

fn get_next_value(data: &[i64]) -> i64 {
    let layers = get_layers(data);
    layers.iter().rev().map(|layer| layer.last().unwrap()).sum()
}

fn get_prev_value(data: &[i64]) -> i64 {
    let layers = get_layers(data);
    let firsts: Vec<i64> = layers
        .iter()
        .rev()
        .map(|layer| *layer.first().unwrap())
        .collect();
    firsts.into_iter().reduce(|acc, x| x - acc).unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let data = parse_input(&input);
    let sum: i64 = data.iter().map(|x| get_next_value(x)).sum();
    println!("Part 1: {sum}");

    let sum: i64 = data.iter().map(|x| get_prev_value(x)).sum();
    println!("Part 2: {sum}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        String::from(
            "0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45",
        )
    }

    #[test]
    fn test_compute_diff() {
        let start = vec![0, 3, 6, 9, 12, 15];
        let diff = compute_diff(&start);
        assert_eq!(diff, vec![3, 3, 3, 3, 3]);

        let start = vec![1, 3, 6, 10, 15, 21];
        let diff = compute_diff(&start);
        assert_eq!(diff, vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_get_layers() {
        let start = vec![0, 3, 6, 9, 12, 15];
        let layers = get_layers(&start);
        assert_eq!(layers.len(), 3);
        assert_eq!(layers[0], start);
        assert_eq!(layers[1], vec![3, 3, 3, 3, 3]);
        assert_eq!(layers[2], vec![0; 4]);

        let start = vec![1, 3, 6, 10, 15, 21];
        let layers = get_layers(&start);
        assert_eq!(layers.len(), 4);
        assert_eq!(layers[0], start);
        assert_eq!(layers[1], vec![2, 3, 4, 5, 6]);
        assert_eq!(layers[2], vec![1, 1, 1, 1]);
        assert_eq!(layers[3], vec![0; 3]);

        let start = vec![10, 13, 16, 21, 30, 45];
        let layers = get_layers(&start);
        assert_eq!(layers.len(), 5);
        assert_eq!(layers[0], start);
        assert_eq!(layers[1], vec![3, 3, 5, 9, 15]);
        assert_eq!(layers[2], vec![0, 2, 4, 6]);
        assert_eq!(layers[3], vec![2, 2, 2]);
        assert_eq!(layers[4], vec![0; 2]);
    }

    #[test]
    fn test_get_next_value() {
        let start = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(get_next_value(&start), 18);

        let start = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(get_next_value(&start), 28);
    }

    #[test]
    fn test_get_prev_value() {
        let start = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(get_prev_value(&start), -3);

        let start = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(get_prev_value(&start), 0);
    }

    #[test]
    fn test_sample_next() {
        let input = get_input();
        let data: Vec<Vec<i64>> = parse_input(&input);
        let sums: Vec<i64> = data.iter().map(|x| get_next_value(x)).collect();
        assert_eq!(sums.iter().sum::<i64>(), 114);
    }

    #[test]
    fn test_sample_prev() {
        let input = get_input();
        let data: Vec<Vec<i64>> = parse_input(&input);
        let sums: Vec<i64> = data.iter().map(|x| get_prev_value(x)).collect();
        assert_eq!(sums.iter().sum::<i64>(), 2);
    }
}
