use aoc_core::cardinal_direction::CardinalDirection;
use aoc_core::point::Point as PointCore;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::str::Lines;

type Point = PointCore<usize>;

#[derive(Copy, Clone, PartialEq, Eq)]
enum PipeTile {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthEast,
    BendSouthWest,
    Ground,
    Start,
}

impl PipeTile {
    fn get_directions(&self) -> (CardinalDirection, CardinalDirection) {
        match self {
            PipeTile::Vertical => (CardinalDirection::North, CardinalDirection::South),
            PipeTile::Horizontal => (CardinalDirection::East, CardinalDirection::West),
            PipeTile::BendNorthEast => (CardinalDirection::North, CardinalDirection::East),
            PipeTile::BendNorthWest => (CardinalDirection::North, CardinalDirection::West),
            PipeTile::BendSouthEast => (CardinalDirection::South, CardinalDirection::East),
            PipeTile::BendSouthWest => (CardinalDirection::South, CardinalDirection::West),
            // This is known from looking at the input, there's only one instance of S
            PipeTile::Start => (CardinalDirection::East, CardinalDirection::West),
            PipeTile::Ground => panic!("Ground tiles have no exits!"),
        }
    }

    fn to_char(&self, is_path: bool) -> char {
        if is_path {
            match self {
                PipeTile::Vertical => '┃',
                PipeTile::Horizontal => '━',
                PipeTile::BendNorthEast => '┗',
                PipeTile::BendNorthWest => '┛',
                PipeTile::BendSouthEast => '┏',
                PipeTile::BendSouthWest => '┓',
                PipeTile::Start => 'S',
                _ => panic!("Character should never be in path"),
            }
        } else {
            match self {
                PipeTile::Vertical => '│',
                PipeTile::Horizontal => '─',
                PipeTile::BendNorthEast => '└',
                PipeTile::BendNorthWest => '┘',
                PipeTile::BendSouthEast => '┌',
                PipeTile::BendSouthWest => '┐',
                PipeTile::Ground => ' ',
                _ => panic!("Character should never be outside path"),
            }
        }
    }
}

impl From<char> for PipeTile {
    fn from(value: char) -> Self {
        match value {
            '|' => PipeTile::Vertical,
            '-' => PipeTile::Horizontal,
            'L' => PipeTile::BendNorthEast,
            'J' => PipeTile::BendNorthWest,
            'F' => PipeTile::BendSouthEast,
            '7' => PipeTile::BendSouthWest,
            '.' => PipeTile::Ground,
            'S' => PipeTile::Start,
            _ => panic!("Unknown character: {}", value),
        }
    }
}

pub struct PipeComplex {
    /// By y, then by x.
    map: Vec<Vec<PipeTile>>,
    marked: HashSet<Point>,
}

impl PipeComplex {
    pub fn get_midpoint(&self) -> usize {
        self.trace_path().len() / 2
    }

    pub fn mark_outside(&mut self) {
        let mut expanded = self.expand();
        expanded.do_mark_outside();
        for point in expanded.iter_points() {
            if point.x % 2 != 0 || point.y % 2 != 0 {
                continue;
            }
            if expanded.marked.contains(&point) {
                self.marked.insert(Point::new(point.x / 2, point.y / 2));
            }
        }
    }

    pub fn count_unmarked(&self) -> usize {
        self.iter_points()
            .filter(|x| !self.marked.contains(&x))
            .count()
    }

    fn expand(&self) -> Self {
        let mut map: Vec<Vec<PipeTile>> = vec![];
        for line in self.map.iter() {
            let mut row: Vec<PipeTile> = vec![];
            let mut row2: Vec<PipeTile> = vec![];
            for tile in line.iter() {
                row.push(*tile);
                row.push(PipeTile::Horizontal);
                row2.push(PipeTile::Vertical);
                row2.push(PipeTile::Vertical);
            }
            map.push(row);
            map.push(row2);
        }

        PipeComplex {
            map,
            marked: HashSet::new(),
        }
    }

    fn do_mark_outside(&mut self) {
        let path_points = self.trace_path();
        for point in path_points {
            self.marked.insert(point);
        }
        let mut to_test: Vec<Point> = vec![Point::new(0, 0)];
        while let Some(next) = to_test.pop() {
            self.mark(&next);
            let mut inner = vec![];
            for dir in self.iter_adjacent(&next) {
                let adjacent = next.go(&dir);
                if !self.marked.contains(&adjacent) {
                    inner.push(adjacent);
                }
            }
            to_test.append(&mut inner);
        }
    }

    fn mark(&mut self, point: &Point) {
        self.marked.insert(*point);
    }

    fn trace_path(&self) -> Vec<Point> {
        let start = self.get_start();
        let mut path: Vec<Point> = vec![start.clone()];
        // Arbitrarily pick a starting direction, it doesn't matter.
        path.push(self.get_exits(&start).0);
        loop {
            let [previous, current] = path.last_chunk::<2>().unwrap();
            let (a, b) = self.get_exits(current);
            let next = match (a != *previous, b != *previous) {
                (true, false) => a,
                (false, true) => b,
                _ => panic!("Unable to determine next point!"),
            };
            if next == start {
                break;
            }
            path.push(next);
        }
        path
    }

    fn get_exits(&self, point: &Point) -> (Point, Point) {
        let tile = self.get_tile(&point);
        let (dir_a, dir_b) = tile.get_directions();
        let point_a = point.go(&dir_a);
        let point_b = point.go(&dir_b);
        (point_a, point_b)
    }

    fn get_tile(&self, point: &Point) -> &PipeTile {
        &self.map[point.y][point.x]
    }

    fn get_start(&self) -> Point {
        for point in self.iter_points() {
            let tile = self.get_tile(&point);
            if *tile == PipeTile::Start {
                return point;
            }
        }
        panic!("No start tile found!")
    }

    fn iter_adjacent(&self, point: &Point) -> impl Iterator<Item = CardinalDirection> {
        let mut result: Vec<CardinalDirection> = vec![];
        if point.y > 0 {
            result.push(CardinalDirection::North);
        }
        if point.x > 0 {
            result.push(CardinalDirection::West);
        }
        if point.y < self.map.len() - 1 {
            result.push(CardinalDirection::South);
        }
        if point.x < self.map.first().unwrap().len() - 1 {
            result.push(CardinalDirection::East);
        }
        result.into_iter()
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + use<'_> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, _)| Point::new(x, y)))
    }
}

impl From<Lines<'_>> for PipeComplex {
    fn from(value: Lines<'_>) -> Self {
        let mut map: Vec<Vec<PipeTile>> = vec![];
        for line in value.into_iter() {
            let mut row = vec![];
            for char in line.chars() {
                row.push(char.into());
            }
            map.push(row);
        }

        PipeComplex {
            map,
            marked: HashSet::new(),
        }
    }
}

impl Display for PipeComplex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let path = self.trace_path();
        for (y, line) in self.map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let point = Point { x, y };
                if self.marked.contains(&point) && !path.contains(&point) {
                    f.write_char('X')?;
                } else {
                    f.write_char(tile.to_char(path.contains(&point)))?;
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::pipe_complex::PipeComplex;

    const INPUT: &str = r#"..........
.FS-----7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#;

    #[test]
    fn test_inside() {
        let mut pipes: PipeComplex = INPUT.lines().into();
        pipes.mark_outside();
        print!("{}", pipes);
        assert_eq!(pipes.count_unmarked(), 4);
    }
}
