// Snailfish numbers aren't like regular numbers. Instead, every snailfish number is a pair - an
// ordered list of two elements. Each element of the pair can be either a regular number or another
// pair.
//
// Pairs are written as [x,y], where x and y are the elements within the pair. Here are some example
// snailfish numbers, one snailfish number per line:

// [1,2]
// [[1,2],3]
// [9,[8,7]]
// [[1,9],[8,5]]
// [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
// [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
// [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailfishNumber {
    left: Box<SnailfishPart>,
    right: Box<SnailfishPart>,
}

impl SnailfishNumber {
    /// Calculates the magnitude of the snailfish number.
    pub fn magnitude(&self) -> i32 {
        let left_mag = match &*self.left {
            SnailfishPart::Regular(val) => *val,
            SnailfishPart::Snailfish(sf) => sf.magnitude(),
        };
        let right_mag = match &*self.right {
            SnailfishPart::Regular(val) => *val,
            SnailfishPart::Snailfish(sf) => sf.magnitude(),
        };
        3 * left_mag + 2 * right_mag
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishPart {
    Regular(i32),
    Snailfish(Box<SnailfishNumber>),
}

fn split_at_comma(s: &str) -> Result<(&str, &str), String> {
    let mut bracket_depth = 0;
    let chars: Vec<char> = s.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            ',' if bracket_depth == 0 => {
                return Ok((&s[0..i], &s[i + 1..]));
            }
            _ => {}
        }
    }

    Err("No top-level comma found".to_string())
}

impl SnailfishNumber {
    /// Adds two snailfish numbers and reduces the result.
    pub fn add(left: &SnailfishNumber, right: &SnailfishNumber) -> SnailfishNumber {
        let mut result = SnailfishNumber {
            left: Box::new(SnailfishPart::Snailfish(Box::new(left.clone()))),
            right: Box::new(SnailfishPart::Snailfish(Box::new(right.clone()))),
        };
        result.reduce();
        result
    }

    /// Reduces the snailfish number by repeatedly exploding and splitting as needed.
    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
    }

    fn explode(&mut self) -> bool {
        let mut left_add = None;
        let mut right_add = None;
        self.explode_helper(0, &mut left_add, &mut right_add)
    }

    fn explode_helper(
        &mut self,
        depth: usize,
        left_add: &mut Option<i32>,
        right_add: &mut Option<i32>,
    ) -> bool {
        // If we're at depth 4 or more and both parts are regular numbers, explode this pair
        if depth >= 4 {
            if let (SnailfishPart::Regular(left_val), SnailfishPart::Regular(right_val)) =
                (&*self.left, &*self.right)
            {
                *left_add = Some(*left_val);
                *right_add = Some(*right_val);
                return true;
            }
        }

        // Try to explode left side
        if let SnailfishPart::Snailfish(left_sf) = &mut *self.left {
            if left_sf.explode_helper(depth + 1, left_add, right_add) {
                // Left side exploded
                if depth + 1 >= 4 {
                    // Replace the exploded pair with 0
                    self.left = Box::new(SnailfishPart::Regular(0));
                }

                // Propagate right_add to the right side
                if let Some(val) = right_add.take() {
                    Self::add_to_leftmost(&mut self.right, val);
                }
                return true;
            }
        }

        // Try to explode right side
        if let SnailfishPart::Snailfish(right_sf) = &mut *self.right {
            if right_sf.explode_helper(depth + 1, left_add, right_add) {
                // Right side exploded
                if depth + 1 >= 4 {
                    // Replace the exploded pair with 0
                    self.right = Box::new(SnailfishPart::Regular(0));
                }

                // Propagate left_add to the left side
                if let Some(val) = left_add.take() {
                    Self::add_to_rightmost(&mut self.left, val);
                }
                return true;
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        // Try to split left side first
        if Self::split_part(&mut self.left) {
            return true;
        }

        // Try to split right side
        Self::split_part(&mut self.right)
    }

    fn split_part(part: &mut SnailfishPart) -> bool {
        match part {
            SnailfishPart::Regular(val) => {
                if *val >= 10 {
                    let left = *val / 2;
                    let right = (*val + 1) / 2; // This handles rounding up
                    *part = SnailfishPart::Snailfish(Box::new(SnailfishNumber {
                        left: Box::new(SnailfishPart::Regular(left)),
                        right: Box::new(SnailfishPart::Regular(right)),
                    }));
                    return true;
                }
                false
            }
            SnailfishPart::Snailfish(sf) => sf.split(),
        }
    }

    fn add_to_leftmost(part: &mut SnailfishPart, value: i32) {
        match part {
            SnailfishPart::Regular(val) => *val += value,
            SnailfishPart::Snailfish(sf) => Self::add_to_leftmost(&mut sf.left, value),
        }
    }

    fn add_to_rightmost(part: &mut SnailfishPart, value: i32) {
        match part {
            SnailfishPart::Regular(val) => *val += value,
            SnailfishPart::Snailfish(sf) => Self::add_to_rightmost(&mut sf.right, value),
        }
    }
}

impl std::str::FromStr for SnailfishNumber {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if !s.starts_with('[') || !s.ends_with(']') {
            return Err("Snailfish number must be enclosed in brackets".to_string());
        }

        let inner = &s[1..s.len() - 1]; // Remove outer brackets
        let (left_str, right_str) = split_at_comma(inner)?;

        let left = parse_snailfish_part(left_str)?;
        let right = parse_snailfish_part(right_str)?;

        Ok(SnailfishNumber {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

// Helper function to parse a string into a SnailfishPart
fn parse_snailfish_part(s: &str) -> Result<SnailfishPart, String> {
    let s = s.trim();
    if s.starts_with('[') {
        let num: SnailfishNumber = s.parse()?;
        Ok(SnailfishPart::Snailfish(Box::new(num)))
    } else {
        let val: i32 = s.parse().map_err(|_| format!("Invalid number: {}", s))?;
        Ok(SnailfishPart::Regular(val))
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl std::fmt::Display for SnailfishPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishPart::Regular(value) => write!(f, "{}", value),
            SnailfishPart::Snailfish(sf) => write!(f, "{}", sf),
        }
    }
}

fn parse_input(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve_p1(input: &str) -> i32 {
    let snailfish_numbers = parse_input(input);
    let mut sum = snailfish_numbers[0].clone();
    for num in &snailfish_numbers[1..] {
        sum = SnailfishNumber::add(&sum, num);
    }
    sum.magnitude()
}

fn solve_p2(input: &str) -> i32 {
    let snailfish_numbers = parse_input(input);
    let mut max_magnitude = 0;

    for (i, left) in snailfish_numbers.iter().enumerate() {
        for (j, right) in snailfish_numbers.iter().enumerate() {
            if i != j {
                let sum = SnailfishNumber::add(left, right);
                let mag = sum.magnitude();
                if mag > max_magnitude {
                    max_magnitude = mag;
                }
            }
        }
    }

    max_magnitude
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
        assert_eq!(answer, 4140); // The correct answer for the AoC Day 18 example
        let answer = solve_p2(&input);
        assert_eq!(answer, 3993);
    }

    #[test]
    fn test_magnitude_calculation() {
        // Test simple magnitude: [9,1] has magnitude 3*9 + 2*1 = 29
        let sf: SnailfishNumber = "[9,1]".parse().unwrap();
        assert_eq!(sf.magnitude(), 29);

        // Test nested magnitude: [[9,1],[1,9]] has magnitude 3*29 + 2*21 = 129
        let sf: SnailfishNumber = "[[9,1],[1,9]]".parse().unwrap();
        assert_eq!(sf.magnitude(), 129);
    }

    #[test]
    fn test_addition() {
        // Test basic addition without reduction
        let left: SnailfishNumber = "[1,2]".parse().unwrap();
        let right: SnailfishNumber = "[[3,4],5]".parse().unwrap();
        let result = SnailfishNumber::add(&left, &right);
        // Should create [[1,2],[[3,4],5]] then reduce if needed
        println!("Addition result: {}", result);
    }

    #[test]
    fn test_snailfish_parsing() {
        // Test simple pair
        let simple = "[1,2]";
        let sf: SnailfishNumber = simple.parse().unwrap();
        assert_eq!(sf.to_string(), "[1,2]");

        // Test nested pair
        let nested = "[[1,2],3]";
        let sf: SnailfishNumber = nested.parse().unwrap();
        assert_eq!(sf.to_string(), "[[1,2],3]");

        // Test complex nested
        let complex = "[9,[8,7]]";
        let sf: SnailfishNumber = complex.parse().unwrap();
        assert_eq!(sf.to_string(), "[9,[8,7]]");

        // Test very complex nested structure
        let very_complex = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let sf: SnailfishNumber = very_complex.parse().unwrap();
        assert_eq!(sf.to_string(), "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    }

    #[test]
    fn test_snailfish_explode() {
        let input = "[[[[[9,8],1],2],3],4]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        sf.explode();
        assert_eq!(sf.to_string(), "[[[[0,9],2],3],4]");
    }

    #[test]
    fn test_snailfish_split() {
        // Test splitting 10
        let input = "[10,1]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        let result = sf.split();
        assert!(result);
        assert_eq!(sf.to_string(), "[[5,5],1]");

        // Test splitting 11
        let input = "[11,1]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        let result = sf.split();
        assert!(result);
        assert_eq!(sf.to_string(), "[[5,6],1]");

        // Test splitting 12
        let input = "[12,1]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        let result = sf.split();
        assert!(result);
        assert_eq!(sf.to_string(), "[[6,6],1]");

        // Test no split needed
        let input = "[9,8]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        let result = sf.split();
        assert!(!result);
        assert_eq!(sf.to_string(), "[9,8]");

        // Test nested split - should split leftmost
        let input = "[15,11]";
        let mut sf: SnailfishNumber = input.parse().unwrap();
        let result = sf.split();
        assert!(result);
        assert_eq!(sf.to_string(), "[[7,8],11]");
    }
}
