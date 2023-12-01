#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl From<char> for CardinalDirection {
    fn from(input: char) -> Self {
        match input.to_lowercase().next().unwrap() {
            'n' => CardinalDirection::North,
            's' => CardinalDirection::South,
            'e' => CardinalDirection::East,
            'w' => CardinalDirection::West,
            'u' => CardinalDirection::North,
            'd' => CardinalDirection::South,
            'r' => CardinalDirection::East,
            'l' => CardinalDirection::West,
            _a => panic!("Char {} cannot be converted to a cardinal direction!", _a),
        }
    }
}
