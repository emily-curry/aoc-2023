use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Write};
use std::str::Lines;

trait SchematicPartLocation {
    fn to_position(&self) -> (u8, u8);

    fn to_size(&self) -> u8;
}

trait SchematicPartDetection {
    fn get_surrounding_positions(&self) -> Vec<(u8, u8)>;

    fn is_inside(&self, x: &u8, y: &u8) -> bool;
}

impl<T> SchematicPartDetection for T
where
    T: SchematicPartLocation,
{
    fn get_surrounding_positions(&self) -> Vec<(u8, u8)> {
        let (self_x, self_y) = self.to_position();
        let mut surrounding_positions = vec![];
        let x_start = self_x.checked_sub(1).unwrap_or(0);
        for x in x_start..=self_x + self.to_size() {
            surrounding_positions.push((x, self_y + 1));
            if self_y > 0 {
                surrounding_positions.push((x, self_y - 1));
            }
        }
        surrounding_positions.push((self_x + self.to_size(), self_y));
        if self_x > 0 {
            surrounding_positions.push((self_x - 1, self_y));
        }
        surrounding_positions
    }

    fn is_inside(&self, x: &u8, y: &u8) -> bool {
        let (self_x, self_y) = self.to_position();
        for i in self_x..self_x + self.to_size() {
            if x == &i && y == &self_y {
                return true;
            }
        }
        false
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct SchematicPartNumber {
    x: u8,
    y: u8,
    size: u8,
    value: u32,
}

impl SchematicPartLocation for SchematicPartNumber {
    fn to_position(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    fn to_size(&self) -> u8 {
        self.size
    }
}

impl Display for SchematicPartNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value.to_string())
    }
}

pub struct SchematicPartSymbol {
    x: u8,
    y: u8,
    value: char,
}

impl SchematicPartLocation for SchematicPartSymbol {
    fn to_position(&self) -> (u8, u8) {
        (self.x, self.y)
    }

    fn to_size(&self) -> u8 {
        1
    }
}

impl Display for SchematicPartSymbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.value)
    }
}

pub enum SchematicPart {
    Number(SchematicPartNumber),
    Symbol(SchematicPartSymbol),
}

impl Display for SchematicPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchematicPart::Number(a) => a.fmt(f),
            SchematicPart::Symbol(a) => a.fmt(f),
        }
    }
}

impl SchematicPartLocation for SchematicPart {
    fn to_position(&self) -> (u8, u8) {
        match self {
            SchematicPart::Number(a) => a.to_position(),
            SchematicPart::Symbol(a) => a.to_position(),
        }
    }

    fn to_size(&self) -> u8 {
        match self {
            SchematicPart::Number(a) => a.to_size(),
            SchematicPart::Symbol(a) => a.to_size(),
        }
    }
}

pub struct Schematic {
    parts: Vec<SchematicPart>,
    width: u8,
    /// By y, then x
    symbol_map: HashMap<u8, HashMap<u8, char>>,
}

impl Schematic {
    fn new(parts: Vec<SchematicPart>, width: u8) -> Schematic {
        let mut symbol_map = HashMap::new();
        let mut parts_iter = parts
            .iter()
            .filter(|i| matches!(i, SchematicPart::Symbol(_)));
        while let Some(SchematicPart::Symbol(s)) = parts_iter.next() {
            let y = match symbol_map.entry(s.y) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(HashMap::default()),
            };
            y.insert(s.x, s.value);
        }

        Schematic {
            parts,
            width,
            symbol_map,
        }
    }

    pub fn to_part_numbers(&self) -> Vec<u32> {
        let mut result = vec![];
        let mut part_iter = self.as_numbers();
        while let Some(s) = part_iter.next() {
            let mut pos_iter = s.get_surrounding_positions().into_iter();
            while let Some((x, y)) = pos_iter.next() {
                if self.get_symbol_at(&x, &y).is_some() {
                    result.push(s.value);
                    break;
                }
            }
        }

        result
    }

    pub fn to_gear_ratios(&self) -> Vec<u32> {
        let mut result = vec![];
        let mut gear_iter = self.as_symbols();
        while let Some(s) = gear_iter.next() {
            if s.value != '*' {
                continue;
            }
            let mut pos_iter = s.get_surrounding_positions().into_iter();
            let mut surrounding: HashSet<&SchematicPartNumber> = HashSet::new();
            while let Some((x, y)) = pos_iter.next() {
                if let Some(num) = self.get_number_at(&x, &y) {
                    surrounding.insert(num);
                }
            }
            if surrounding.len() == 2 {
                let gear_ratio = surrounding.iter().fold(1u32, |acc, val| acc * val.value);
                result.push(gear_ratio);
            }
        }

        result
    }

    fn get_number_at(&self, x: &u8, y: &u8) -> Option<&SchematicPartNumber> {
        self.as_numbers().find(|i| i.is_inside(x, y))
    }

    fn get_symbol_at(&self, x: &u8, y: &u8) -> Option<&SchematicPartSymbol> {
        self.as_symbols().find(|i| i.is_inside(x, y))
    }

    fn as_numbers(&self) -> impl Iterator<Item = &SchematicPartNumber> {
        self.parts.iter().filter_map(|i| match i {
            SchematicPart::Number(n) => Some(n),
            _ => None,
        })
    }

    fn as_symbols(&self) -> impl Iterator<Item = &SchematicPartSymbol> {
        self.parts.iter().filter_map(|i| match i {
            SchematicPart::Symbol(n) => Some(n),
            _ => None,
        })
    }
}

impl From<Lines<'_>> for Schematic {
    fn from(value: Lines) -> Self {
        let mut parts: Vec<SchematicPart> = vec![];
        let mut number_buffer = String::from("");
        let mut width = 0u8;
        for (y, line) in value.enumerate() {
            width = max(width, line.len() as u8);
            for (x, c) in line.chars().enumerate() {
                if c.is_numeric() {
                    number_buffer.push(c);
                    continue;
                }
                if !&number_buffer.is_empty() {
                    let size = number_buffer.len() as u8;
                    parts.push(SchematicPart::Number(SchematicPartNumber {
                        x: (x as u8) - size,
                        y: y as u8,
                        size,
                        value: number_buffer.parse().unwrap(),
                    }));
                    number_buffer.clear();
                }
                if c == '.' {
                    continue;
                }
                parts.push(SchematicPart::Symbol(SchematicPartSymbol {
                    x: x as u8,
                    y: y as u8,
                    value: c,
                }));
            }
            if !&number_buffer.is_empty() {
                let size = number_buffer.len() as u8;
                parts.push(SchematicPart::Number(SchematicPartNumber {
                    x: (line.len() as u8) - size,
                    y: y as u8,
                    size,
                    value: number_buffer.parse().unwrap(),
                }));
                number_buffer.clear();
            }
        }
        Schematic::new(parts, width + 1)
    }
}

/// Just seeing if I can reproduce the input.
impl Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parts = self.parts.iter();
        let mut x_pos = 0u8;
        let mut y_pos = 0u8;
        while let Some(n) = parts.next() {
            let (x, y) = n.to_position();
            // write dots up until the next part
            while y_pos < y {
                while x_pos < self.width {
                    f.write_char('.')?;
                    x_pos += 1;
                }
                f.write_char('\n')?;
                x_pos = 0;
                y_pos += 1;
            }
            while x_pos < x {
                f.write_char('.')?;
                x_pos += 1;
            }
            n.fmt(f)?;
            x_pos += n.to_size();
        }
        Ok(())
    }
}
