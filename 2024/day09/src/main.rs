#[derive(Debug, Clone, Copy)]
enum BlockType {
    Data(u32),
    Space,
}

impl BlockType {
    fn is_data(&self) -> bool {
        matches!(self, BlockType::Data(_))
    }

    fn is_space(&self) -> bool {
        matches!(self, BlockType::Space)
    }

    fn get_data(&self) -> u32 {
        match self {
            BlockType::Data(data) => *data,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Block {
    blk_type: BlockType,
    start: usize,
    end: usize,
}

impl Block {
    fn new(blk_type: BlockType, start: usize, end: usize) -> Self {
        Self {
            blk_type,
            start,
            end,
        }
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

fn parse_block_input(input: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut id = 0;
    input.chars().enumerate().for_each(|(i, ch)| {
        let count = ch.to_digit(10).unwrap() as usize;
        start = end;
        end = start + count;

        if i % 2 == 0 {
            blocks.push(Block::new(BlockType::Data(id), start, end));
            id += 1;
        } else {
            blocks.push(Block::new(BlockType::Space, start, end));
        }
    });
    blocks
}

fn parse_input(input: &str) -> Vec<char> {
    input.trim().chars().collect()
}

#[allow(dead_code)]
fn display(data: &[u32]) {
    data.iter().for_each(|&x| {
        print!("{x}");
    });
    println!();
}

#[allow(dead_code)]
fn display_blocks(blocks: &[Block]) {
    blocks.iter().for_each(|blk| {
        if blk.blk_type.is_data() {
            print!(
                "{}",
                (0..blk.len())
                    .map(|_| blk.blk_type.get_data().to_string())
                    .collect::<String>()
            );
        } else {
            print!("{}", (0..blk.len()).map(|_| ".").collect::<String>());
        }
    });
    println!();
}

fn solve_p1(input: &str) -> isize {
    let data = parse_input(input);
    let mut decoded: Vec<isize> = Vec::new();
    let mut id = 0;
    data.iter().enumerate().for_each(|(i, ch)| {
        let count = ch.to_digit(10).unwrap();
        if i % 2 == 0 {
            (0..count).for_each(|_| decoded.push(id));
            id += 1;
        } else {
            (0..count).for_each(|_| decoded.push(-1));
        }
    });

    // compact
    let mut fwd_idx = 0;
    let mut rev_idx = decoded.len() - 1;
    let mut checksum = 0;
    while fwd_idx <= rev_idx {
        if decoded[fwd_idx] != -1 {
            checksum += fwd_idx as isize * decoded[fwd_idx];
            fwd_idx += 1;
            continue;
        }
        if decoded[rev_idx] == -1 {
            rev_idx -= 1;
            continue;
        }
        decoded.swap(fwd_idx, rev_idx);
        checksum += fwd_idx as isize * decoded[fwd_idx];
        fwd_idx += 1;
        rev_idx -= 1;
    }
    checksum
}

fn solve_p2(input: &str) -> isize {
    let mut blocks = parse_block_input(input);

    let mut blk_idx = blocks.len() - 1;
    while blk_idx > 0 {
        let blk = blocks[blk_idx];
        if blk.blk_type.is_space() {
            blk_idx -= 1;
            continue;
        }
        let max_space_idx = blk_idx;
        for space_idx in 0..max_space_idx {
            if blocks[space_idx].blk_type.is_space() {
                let space_len = blocks[space_idx].len();
                if space_len < blk.len() {
                    continue;
                }
                // Found a space, move the data block to the space
                let moved_start = blocks[space_idx].start;
                let moved_end = blocks[space_idx].start + blk.len();
                blocks[space_idx].start += blk.len();
                blocks.insert(space_idx, Block::new(blk.blk_type, moved_start, moved_end));
                blk_idx += 1; // To account for the inserted block
                blocks[blk_idx].blk_type = BlockType::Space;

                // Collapse consecutive spaces
                if blk_idx + 1 < blocks.len() {
                    if blocks[blk_idx + 1].blk_type.is_space() {
                        blocks[blk_idx].end = blocks[blk_idx + 1].end;
                        blocks.remove(blk_idx + 1);
                    }
                    if blocks[blk_idx - 1].blk_type.is_space() {
                        blocks[blk_idx - 1].end = blocks[blk_idx].end;
                        blocks.remove(blk_idx);
                        blk_idx -= 1;
                    }
                }
                break;
            }
        }
        blk_idx -= 1;
    }

    let mut checksum = 0;
    let mut pos: isize = 0;
    blocks.iter().for_each(|blk| {
        if blk.blk_type.is_data() {
            (0..blk.len()).for_each(|_| {
                checksum += pos * blk.blk_type.get_data() as isize;
                pos += 1;
            });
        } else {
            pos += blk.len() as isize;
        }
    });
    checksum
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
        let input = "2333133121414131402";
        let answer = solve_p1(&input);
        assert_eq!(answer, 1928);
        let answer = solve_p2(&input);
        assert_eq!(answer, 2858);
    }
}
