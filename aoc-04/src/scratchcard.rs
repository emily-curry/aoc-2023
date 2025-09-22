use std::collections::HashSet;
use std::str::Lines;

pub struct Scratchcard {
    id: u8,
    winners: HashSet<u8>,
    numbers: Vec<u8>,
    copies: u32,
}

impl Scratchcard {
    fn to_points(&self) -> u32 {
        let winner_count = self.to_winner_count();
        if winner_count == 0 {
            0
        } else {
            2u32.pow(winner_count - 1)
        }
    }

    fn to_winner_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|i| self.winners.contains(i))
            .count() as u32
    }
}

impl From<&str> for Scratchcard {
    fn from(value: &str) -> Self {
        let mut split = value.split(|c| c == ':' || c == '|');
        let id: u8 = split
            .next()
            .unwrap()
            .replace("Card", "")
            .replace(" ", "")
            .parse()
            .unwrap();
        let winners: HashSet<u8> = split
            .next()
            .unwrap()
            .split(" ")
            .filter(|i| !i.is_empty())
            .map(|i| i.parse().unwrap())
            .collect();
        let numbers: Vec<u8> = split
            .next()
            .unwrap()
            .split(" ")
            .filter(|i| !i.is_empty())
            .map(|i| i.parse().unwrap())
            .collect();

        Scratchcard {
            id,
            winners,
            numbers,
            copies: 1,
        }
    }
}

pub struct ScratchcardStack {
    scratchcards: Vec<Scratchcard>,
}

impl ScratchcardStack {
    pub fn to_point_sum(&self) -> u32 {
        self.scratchcards.iter().map(Scratchcard::to_points).sum()
    }

    pub fn to_copies_sum(&self) -> u32 {
        self.scratchcards.iter().map(|i| i.copies).sum()
    }

    fn count_copies(&mut self) {
        for i in 0..self.scratchcards.len() {
            let item = self.scratchcards.get(i).unwrap();
            let copies = item.copies;
            for k in i + 1..=i + (item.to_winner_count() as usize) {
                if let Some(c) = self.scratchcards.get_mut(k) {
                    c.copies += 1 * copies;
                }
            }
        }
    }
}

impl From<Lines<'_>> for ScratchcardStack {
    fn from(value: Lines) -> Self {
        let mut stack = ScratchcardStack {
            scratchcards: value.into_iter().map(Scratchcard::from).collect(),
        };
        stack.count_copies();

        stack
    }
}
