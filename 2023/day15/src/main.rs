use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct LensBox {
    id: u32,
    lenses: Vec<Lens>,
}

impl LensBox {
    fn new(id: u32) -> Self {
        LensBox {
            id,
            lenses: Vec::with_capacity(9),
        }
    }

    fn focusing_power(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(slot, lens)| (self.id + 1) * (slot as u32 + 1) * lens.focal_length)
            .sum()
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn parse_input(input: &str) -> Vec<String> {
    input.trim().split(',').map(String::from).collect()
}

fn hash(step: &str) -> usize {
    step.chars()
        .fold(0, |curr, c| ((curr + (c as usize)) * 17) % 256)
}

fn process(steps: &[String], boxes: &mut [LensBox]) {
    steps.iter().for_each(|step| {
        if step.contains('-') {
            remove_operation(boxes, step);
        } else if step.contains('=') {
            add_operation(boxes, step);
        } else {
            panic!("Unsupported operation {step}");
        }
    });
}

fn remove_operation(boxes: &mut [LensBox], op: &str) {
    let label = op.split_once('-').unwrap().0;
    boxes[hash(label)].lenses.retain(|lens| lens.label != label);
}

fn add_operation(boxes: &mut [LensBox], op: &str) {
    let (label, focal_length) = op.split_once('=').unwrap();
    let lens = Lens {
        label: label.to_owned(),
        focal_length: focal_length.parse::<u32>().unwrap(),
    };
    let lbox = &mut boxes[hash(label)];
    if let Some(existing_lens) = lbox.lenses.iter().position(|lens| lens.label == label) {
        lbox.lenses[existing_lens] = lens;
    } else {
        lbox.lenses.push(lens);
    }
}

fn solve_p1(input: &str) -> usize {
    let steps = parse_input(input);
    steps.iter().map(|step| hash(step)).sum()
}

fn solve_p2(input: &str) -> u32 {
    let steps = parse_input(input);
    let mut boxes: Vec<_> = (0..256).map(LensBox::new).collect();
    process(&steps, &mut boxes);
    boxes.iter().map(|b| b.focusing_power()).sum()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let answer = solve_p1(&input);
    println!("Part 1: {answer}");

    let answer = solve_p2(&input);
    println!("Part 1: {answer}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        String::from("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7")
    }

    #[test]
    fn test_parse_input() {
        let input = get_test_input();
        let steps = parse_input(&input);
        assert_eq!(steps.len(), 11);
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("H"), 200);
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_box_focusing_power() {
        let mut b = LensBox::new(0);
        b.lenses.push(Lens {
            label: "rn".to_owned(),
            focal_length: 1,
        });
        assert_eq!(b.focusing_power(), 1);
        b.lenses.push(Lens {
            label: "cm".to_owned(),
            focal_length: 2,
        });
        assert_eq!(b.focusing_power(), 5);
    }

    #[test]
    fn solve_with_test_input() {
        let input = get_test_input();
        let answer = solve_p1(&input);
        assert_eq!(answer, 1320);
        let answer = solve_p2(&input);
        assert_eq!(answer, 145);
    }

    #[test]
    fn solve() -> Result<()> {
        let input = fs::read_to_string("input.txt")?;
        let answer = solve_p1(&input);
        assert_eq!(answer, 516804);
        let answer = solve_p2(&input);
        assert_eq!(answer, 231844);

        Ok(())
    }
}
