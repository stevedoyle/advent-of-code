use std::fmt;
use std::ops::{Index, IndexMut};

use crate::Point;

/// Dense 2D grid stored row-major in a single `Vec`, indexed by [`Point`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub width: i64,
    pub height: i64,
    pub cells: Vec<T>,
}

impl Grid<u8> {
    /// Parse a character grid as raw bytes (compare with `b'#'` etc.).
    pub fn parse(input: &str) -> Self {
        Self::parse_with(input, |b| b)
    }
}

impl<T> Grid<T> {
    /// Parse a grid, mapping each byte, e.g. digits: `Grid::parse_with(s, |b| (b - b'0') as u32)`.
    pub fn parse_with(input: &str, f: impl FnMut(u8) -> T) -> Self {
        let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
        let width = lines.first().map_or(0, |l| l.len());
        assert!(
            lines.iter().all(|l| l.len() == width),
            "grid rows have different lengths"
        );
        Self {
            width: width as i64,
            height: lines.len() as i64,
            cells: lines.iter().flat_map(|l| l.bytes()).map(f).collect(),
        }
    }

    pub fn new(width: i64, height: i64, fill: T) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            cells: vec![fill; (width * height) as usize],
        }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.width && p.y < self.height
    }

    fn idx(&self, p: Point) -> usize {
        (p.y * self.width + p.x) as usize
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.contains(p).then(|| &self.cells[self.idx(p)])
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        if self.contains(p) {
            let i = self.idx(p);
            Some(&mut self.cells[i])
        } else {
            None
        }
    }

    /// All positions in row-major order.
    pub fn points(&self) -> impl Iterator<Item = Point> {
        let (w, h) = (self.width, self.height);
        (0..h).flat_map(move |y| (0..w).map(move |x| Point::new(x, y)))
    }

    /// `(position, value)` pairs in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = (Point, &T)> {
        self.points().zip(self.cells.iter())
    }

    pub fn find(&self, pred: impl Fn(&T) -> bool) -> Option<Point> {
        self.iter().find(|&(_, v)| pred(v)).map(|(p, _)| p)
    }

    pub fn find_all(&self, pred: impl Fn(&T) -> bool) -> Vec<Point> {
        self.iter()
            .filter(|&(_, v)| pred(v))
            .map(|(p, _)| p)
            .collect()
    }

    /// In-bounds orthogonal neighbours of `p`.
    pub fn neighbors4(&self, p: Point) -> impl Iterator<Item = Point> {
        p.neighbors4().filter(move |&q| self.contains(q))
    }

    /// In-bounds neighbours of `p`, including diagonals.
    pub fn neighbors8(&self, p: Point) -> impl Iterator<Item = Point> {
        p.neighbors8().filter(move |&q| self.contains(q))
    }

    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            cells: self.cells.iter().map(f).collect(),
        }
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    fn index(&self, p: Point) -> &T {
        assert!(self.contains(p), "{p} out of bounds");
        &self.cells[self.idx(p)]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    fn index_mut(&mut self, p: Point) -> &mut T {
        assert!(self.contains(p), "{p} out of bounds");
        let i = self.idx(p);
        &mut self.cells[i]
    }
}

/// Print a byte grid as text — handy for debugging.
impl fmt::Display for Grid<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.cells.chunks(self.width.max(1) as usize) {
            writeln!(f, "{}", String::from_utf8_lossy(row))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_index() {
        let g = Grid::parse("ab\ncd\n");
        assert_eq!((g.width, g.height), (2, 2));
        assert_eq!(g[Point::new(1, 0)], b'b');
        assert_eq!(g.get(Point::new(0, 1)), Some(&b'c'));
        assert_eq!(g.get(Point::new(2, 0)), None);
        assert_eq!(g.get(Point::new(-1, 0)), None);
        assert_eq!(g.to_string(), "ab\ncd\n");
    }

    #[test]
    fn find_and_neighbors() {
        let g = Grid::parse_with("123\n456", |b| (b - b'0') as u32);
        assert_eq!(g.find(|&v| v == 5), Some(Point::new(1, 1)));
        assert_eq!(g.find_all(|&v| v % 2 == 0).len(), 3);
        assert_eq!(g.neighbors4(Point::ORIGIN).count(), 2);
        assert_eq!(g.neighbors8(Point::new(1, 0)).count(), 5);
    }
}
