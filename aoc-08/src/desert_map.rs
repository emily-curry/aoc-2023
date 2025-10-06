use aoc_core::cardinal_direction::CardinalDirection;
use aoc_core::num::LeastCommonMultiple;
use std::collections::HashMap;
use std::str::Lines;

struct DesertMapPeriod {
    start: String,
    /// Originally this struct also included the offset from the start of the list to the repeating
    /// element, as well as the offset from the repeating element to the item ending in 'Z'. The
    /// input data ended up having an interesting property where these end up being irrelevant,
    /// and we can measure the period only.
    period: u64,
}

pub struct DesertMap {
    instructions: Vec<CardinalDirection>,
    nodes: HashMap<String, (String, String)>,
}

impl DesertMap {
    pub fn navigate(&self) -> u64 {
        let mut current: &str = "AAA";
        let mut steps = 0u64;
        let mut iter = self.instructions.iter();
        while current != "ZZZ" {
            let next_instruction = iter.next();
            if let Some(dir) = next_instruction {
                steps += 1;
                current = self.get_next(current, dir);
            } else {
                iter = self.instructions.iter();
            }
        }

        steps
    }

    pub fn navigate_ghost(&self) -> u64 {
        let starts: Vec<&String> = self.nodes.keys().filter(|x| x.ends_with('A')).collect();
        let mut periods = starts.iter().map(|x| self.find_period(x));
        let first = periods.next().unwrap();
        periods.fold(first.period, |acc, val| acc.lcm(&val.period))
    }

    fn find_period(&self, start: &str) -> DesertMapPeriod {
        let mut visited: Vec<(usize, &str)> = vec![(0, &start)];
        let mut current: &str = start;
        let mut iter = self.instructions.iter().enumerate();
        loop {
            let next_instruction = iter.next();
            if let Some((i, dir)) = next_instruction {
                current = self.get_next(current, dir);
                if visited.contains(&(i, &current)) {
                    visited.push((i, current));
                    break;
                }
                visited.push((i, current));
            } else {
                iter = self.instructions.iter().enumerate();
            }
        }
        let repeat = visited.pop().unwrap();
        let offset = visited
            .iter()
            .enumerate()
            .find(|(_, v)| **v == repeat)
            .unwrap()
            .0 as u64;
        // This ends up being always identical to the distance to the first "Z" entry.
        let period = visited.len() as u64 - offset;
        DesertMapPeriod {
            start: start.to_owned(),
            period,
        }
    }

    fn get_next(&self, current: &str, dir: &CardinalDirection) -> &String {
        let (left, right) = self.nodes.get(current).unwrap();
        match dir {
            CardinalDirection::East => right,
            CardinalDirection::West => left,
            _ => panic!("Unexpected direction!"),
        }
    }
}

impl From<Lines<'_>> for DesertMap {
    fn from(mut value: Lines<'_>) -> Self {
        let instructions = value
            .next()
            .unwrap()
            .chars()
            .map(CardinalDirection::from)
            .collect();
        value.next();
        let mut nodes = HashMap::new();
        for line in value {
            let mut split = line.split(" = (");
            let key = split.next().unwrap().to_owned();
            split = split.next().unwrap().split(", ");
            let left = split.next().unwrap().to_owned();
            let right = split.next().unwrap().replace(')', "");
            nodes.insert(key, (left, right));
        }

        DesertMap {
            instructions,
            nodes,
        }
    }
}
