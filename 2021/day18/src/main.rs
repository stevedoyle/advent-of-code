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

/// Type representing the magnitude of a snailfish number
pub type Magnitude = i32;

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailfishNumber {
    left: Box<SnailfishPart>,
    right: Box<SnailfishPart>,
}

impl SnailfishNumber {
    /// Calculates the magnitude of the snailfish number.
    pub fn magnitude(&self) -> Magnitude {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SnailfishPart {
    Regular(i32),
    Snailfish(Box<SnailfishNumber>),
}

impl SnailfishPart {
    fn magnitude(&self) -> Magnitude {
        match self {
            SnailfishPart::Regular(val) => *val,
            SnailfishPart::Snailfish(sf) => sf.magnitude(),
        }
    }

    /// Adds a value to the leftmost regular number in this subtree.
    ///
    /// Used during explosion to propagate values to neighboring regular numbers.
    /// Recursively traverses left until it finds a regular number.
    fn add_to_leftmost(&mut self, value: i32) {
        match self {
            SnailfishPart::Regular(val) => *val += value,
            SnailfishPart::Snailfish(sf) => sf.left.add_to_leftmost(value),
        }
    }

    /// Adds a value to the rightmost regular number in this subtree.
    ///
    /// Used during explosion to propagate values to neighboring regular numbers.
    /// Recursively traverses right until it finds a regular number.
    fn add_to_rightmost(&mut self, value: i32) {
        match self {
            SnailfishPart::Regular(val) => *val += value,
            SnailfishPart::Snailfish(sf) => sf.right.add_to_rightmost(value),
        }
    }
}

fn split_at_comma(s: &str) -> Result<(&str, &str), String> {
    let mut bracket_depth = 0;

    for (i, ch) in s.char_indices() {
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
    ///
    /// Creates a new snailfish number pair [left, right] and then reduces it
    /// according to the snailfish arithmetic rules.
    pub fn add(left: &SnailfishNumber, right: &SnailfishNumber) -> SnailfishNumber {
        let mut result = SnailfishNumber {
            left: Box::new(SnailfishPart::Snailfish(Box::new(left.clone()))),
            right: Box::new(SnailfishPart::Snailfish(Box::new(right.clone()))),
        };
        result.reduce();
        result
    }

    /// Reduces the snailfish number by repeatedly exploding and splitting as needed.
    ///
    /// Reduction process:
    /// 1. **Explode first**: Try to explode any pair nested 4+ levels deep
    /// 2. **Split second**: If no explosion, try to split any regular number >= 10  
    /// 3. **Repeat**: Continue until no more reductions are possible
    ///
    /// # Priority Order
    /// Explosions always take priority over splits. This means if both operations
    /// are possible, explode first. Only when no explosions are possible should
    /// splitting occur. This ordering is crucial for correct snailfish arithmetic.
    ///
    /// # Termination
    /// The loop terminates when neither explosion nor splitting is possible,
    /// indicating the snailfish number is in its fully reduced form.
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

    /// Attempts to explode the first pair nested 4 or more levels deep.
    ///
    /// Explosion rules:
    /// - Find the leftmost pair nested at depth 4 or greater
    /// - The pair must consist of two regular numbers
    /// - Left value is added to the first regular number to the left (if any)
    /// - Right value is added to the first regular number to the right (if any)
    /// - The exploding pair is replaced with the regular number 0
    ///
    /// # Example
    /// ```text
    /// [[[[[9,8],1],2],3],4]
    ///     ^^^^^ explodes (at depth 4)
    ///
    /// Process:
    /// 1. [9,8] is at depth 4, both are regular numbers → explode
    /// 2. 9 has no regular number to its left → ignored
    /// 3. 8 is added to 1 (first regular number to right) → 1 becomes 9
    /// 4. [9,8] is replaced with 0
    ///
    /// Result: [[[[0,9],2],3],4]
    /// ```
    ///
    /// Returns true if an explosion occurred, false otherwise.
    fn explode(&mut self) -> bool {
        let mut left_add = None; // Value to add to the left neighbor
        let mut right_add = None; // Value to add to the right neighbor
        self.explode_helper(0, &mut left_add, &mut right_add)
    }

    /// Recursive helper for explosion detection and handling.
    ///
    /// This function performs a depth-first traversal to find pairs that need to explode.
    /// It uses mutable references to `left_add` and `right_add` to carry values that need
    /// to be propagated to neighboring regular numbers.
    ///
    /// # Algorithm Flow
    /// 1. **Detection**: Check if current pair should explode (depth >= 4, both regular)
    /// 2. **Left subtree**: Try to explode in left child, handle propagation
    /// 3. **Right subtree**: Try to explode in right child, handle propagation
    /// 4. **Propagation**: Values bubble up until they find appropriate neighbors
    ///
    /// # Arguments
    /// * `depth` - Current nesting depth (root is 0)
    /// * `left_add` - Value to add to the next regular number found on the left
    /// * `right_add` - Value to add to the next regular number found on the right
    fn explode_helper(
        &mut self,
        depth: usize,
        left_add: &mut Option<i32>,
        right_add: &mut Option<i32>,
    ) -> bool {
        const EXPLODE_DEPTH: usize = 4;

        // Check if this pair should explode:
        // 1. We're at depth 4 or deeper
        // 2. Both left and right are regular numbers (not nested pairs)
        if depth >= EXPLODE_DEPTH
            && let (SnailfishPart::Regular(left_val), SnailfishPart::Regular(right_val)) =
                (&*self.left, &*self.right)
        {
            // Store the values for propagation to neighbors
            *left_add = Some(*left_val); // Will be added to left neighbor
            *right_add = Some(*right_val); // Will be added to right neighbor
            return true; // Signal that explosion occurred
        }

        // Try to explode something in the left subtree
        if let SnailfishPart::Snailfish(left_sf) = &mut *self.left
            && left_sf.explode_helper(depth + 1, left_add, right_add)
        {
            // An explosion occurred in the left subtree

            // If the explosion was at exactly the threshold depth, replace the
            // exploded pair with 0 (this happens when we're the direct parent)
            if depth + 1 >= EXPLODE_DEPTH {
                self.left = Box::new(SnailfishPart::Regular(0));
            }

            // The right_add value needs to be propagated to the first regular number
            // to the right of the explosion site. Since we're in the left subtree,
            // we need to add it to the leftmost regular number in our right subtree.
            if let Some(val) = right_add.take() {
                self.right.add_to_leftmost(val);
            }

            // left_add remains in the option to be handled by our parent
            return true; // Signal explosion occurred
        }

        // Try to explode something in the right subtree
        if let SnailfishPart::Snailfish(right_sf) = &mut *self.right
            && right_sf.explode_helper(depth + 1, left_add, right_add)
        {
            // An explosion occurred in the right subtree

            // If the explosion was at exactly the threshold depth, replace the
            // exploded pair with 0 (this happens when we're the direct parent)
            if depth + 1 >= EXPLODE_DEPTH {
                self.right = Box::new(SnailfishPart::Regular(0));
            }

            // The left_add value needs to be propagated to the first regular number
            // to the left of the explosion site. Since we're in the right subtree,
            // we need to add it to the rightmost regular number in our left subtree.
            if let Some(val) = left_add.take() {
                self.left.add_to_rightmost(val);
            }

            // right_add remains in the option to be handled by our parent
            return true; // Signal explosion occurred
        }

        // No explosion occurred in this subtree
        false
    }

    /// Attempts to split the first regular number that is 10 or greater.
    ///
    /// Split rules:
    /// - Find the leftmost regular number >= 10
    /// - Replace it with a pair: [left, right]
    /// - Left element = original value ÷ 2 (rounded down)
    /// - Right element = original value ÷ 2 (rounded up)
    ///
    /// # Examples
    /// ```text
    /// 10 → [5,5]     (10/2 = 5, 10/2 = 5)
    /// 11 → [5,6]     (11/2 = 5, 12/2 = 6)
    /// 12 → [6,6]     (12/2 = 6, 12/2 = 6)
    /// 15 → [7,8]     (15/2 = 7, 16/2 = 8)
    /// ```
    ///
    /// The algorithm processes left-to-right to ensure the leftmost eligible
    /// number is split first, which is required by snailfish arithmetic rules.
    ///
    /// Returns true if a split occurred, false otherwise.
    fn split(&mut self) -> bool {
        // Try to split left side first (leftmost priority)
        if Self::split_part(&mut self.left) {
            return true;
        }

        // Try to split right side only if left didn't split
        Self::split_part(&mut self.right)
    }

    /// Recursive helper that attempts to split a single SnailfishPart.
    ///
    /// This function handles the actual splitting logic and recursive traversal.
    /// It follows a left-to-right, depth-first search to find the first
    /// regular number >= 10.
    ///
    /// # Splitting Algorithm
    /// 1. **Regular numbers**: Check if >= 10, split if true
    /// 2. **Nested pairs**: Recursively search for splittable numbers
    /// 3. **Rounding**: Uses integer division with careful rounding
    ///    - Left = `val / 2` (automatic floor division)
    ///    - Right = `(val + 1) / 2` (ceiling division via +1 trick)
    ///
    /// # Mathematical Examples
    /// ```text
    /// Value 10: left = 10/2 = 5,     right = (10+1)/2 = 5
    /// Value 11: left = 11/2 = 5,     right = (11+1)/2 = 6  
    /// Value 15: left = 15/2 = 7,     right = (15+1)/2 = 8
    /// Value 20: left = 20/2 = 10,    right = (20+1)/2 = 10
    /// ```
    fn split_part(part: &mut SnailfishPart) -> bool {
        const SPLIT_THRESHOLD: i32 = 10;

        match part {
            SnailfishPart::Regular(val) => {
                // Check if this regular number needs splitting
                if *val >= SPLIT_THRESHOLD {
                    // Calculate split values with proper rounding
                    let left = *val / 2; // Floor division (round down)
                    let right = (*val + 1) / 2; // Ceiling division (round up)

                    // Replace the regular number with a new snailfish pair
                    *part = SnailfishPart::Snailfish(Box::new(SnailfishNumber {
                        left: Box::new(SnailfishPart::Regular(left)),
                        right: Box::new(SnailfishPart::Regular(right)),
                    }));
                    return true; // Signal that split occurred
                }
                false // No split needed
            }
            SnailfishPart::Snailfish(sf) => {
                // Recursively try to split within this nested snailfish number
                sf.split()
            }
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

fn parse_input(input: &str) -> Result<Vec<SnailfishNumber>, String> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::parse)
        .collect()
}

fn parse_input_unwrap(input: &str) -> Vec<SnailfishNumber> {
    parse_input(input).expect("Failed to parse input")
}

fn solve_p1(input: &str) -> i32 {
    let snailfish_numbers = parse_input_unwrap(input);
    let mut sum = snailfish_numbers[0].clone();
    for num in &snailfish_numbers[1..] {
        sum = SnailfishNumber::add(&sum, num);
    }
    sum.magnitude()
}

fn solve_p2(input: &str) -> i32 {
    let snailfish_numbers = parse_input_unwrap(input);
    let mut max_magnitude = 0;

    for (i, left) in snailfish_numbers.iter().enumerate() {
        for (j, right) in snailfish_numbers.iter().enumerate() {
            if i != j {
                let sum = SnailfishNumber::add(left, right);
                let mag = sum.magnitude();
                max_magnitude = max_magnitude.max(mag);
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
