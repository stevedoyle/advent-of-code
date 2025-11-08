use pathfinding::matrix::Matrix;

fn parse_input(input: &str) -> (String, Matrix<char>) {
    let mut lines = input.lines();

    // Read algorithm (may span multiple lines until we hit an empty line)
    let mut algorithm = String::new();
    for line in lines.by_ref() {
        if line.trim().is_empty() {
            break;
        }
        algorithm.push_str(line);
    }

    // Read image lines
    let image_lines: Vec<Vec<char>> = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let image = Matrix::from_rows(image_lines).unwrap();
    (algorithm, image)
}

fn enhance_image(algorithm: &str, image: &Matrix<char>, default_char: char) -> Matrix<char> {
    let mut enhanced = Matrix::new(image.rows + 2, image.columns + 2, '.');

    for r in 0..enhanced.rows {
        for c in 0..enhanced.columns {
            let mut index = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    index <<= 1;
                    let nr = r as isize + dr - 1;
                    let nc = c as isize + dc - 1;
                    let pixel = if nr >= 0
                        && nr < image.rows as isize
                        && nc >= 0
                        && nc < image.columns as isize
                    {
                        image[(nr as usize, nc as usize)]
                    } else {
                        default_char
                    };

                    if pixel == '#' {
                        index |= 1;
                    }
                }
            }
            enhanced[(r, c)] = algorithm.chars().nth(index).unwrap();
        }
    }

    enhanced
}

fn count_lit_pixels(image: &Matrix<char>) -> i32 {
    image.items().filter(|(_, c)| **c == '#').count() as i32
}

fn solve_p1(input: &str) -> i32 {
    let (algorithm, mut image) = parse_input(input);

    // The background starts as '.' (dark)
    let mut background = '.';

    // Enhance twice
    for _ in 0..2 {
        image = enhance_image(&algorithm, &image, background);

        // After enhancement, the background might flip
        // If algorithm[0] is '#', then all-dark pixels become lit
        // If algorithm[511] is '.', then all-lit pixels become dark
        background = if background == '.' {
            algorithm.chars().next().unwrap()
        } else {
            algorithm.chars().nth(511).unwrap()
        };
    }

    count_lit_pixels(&image)
}

fn solve_p2(input: &str) -> i32 {
    let (algorithm, mut image) = parse_input(input);

    // The background starts as '.' (dark)
    let mut background = '.';

    // Enhance 50 times for part 2
    for _ in 0..50 {
        image = enhance_image(&algorithm, &image, background);

        // After enhancement, the background might flip
        background = if background == '.' {
            algorithm.chars().next().unwrap()
        } else {
            algorithm.chars().nth(511).unwrap()
        };
    }

    count_lit_pixels(&image)
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

    #[test]
    fn test_solve_with_test_input() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer = solve_p1(&input);
        assert_eq!(answer, 35);
        let answer = solve_p2(&input);
        assert_eq!(answer, 3351);
    }
}
