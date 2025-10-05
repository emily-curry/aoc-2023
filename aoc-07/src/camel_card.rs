use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::Lines;

#[derive(Eq, PartialEq)]
enum CamelCardHandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<&CamelCardHandType> for u8 {
    fn from(value: &CamelCardHandType) -> Self {
        match value {
            CamelCardHandType::FiveKind => 6,
            CamelCardHandType::FourKind => 5,
            CamelCardHandType::FullHouse => 4,
            CamelCardHandType::ThreeKind => 3,
            CamelCardHandType::TwoPair => 2,
            CamelCardHandType::OnePair => 1,
            CamelCardHandType::HighCard => 0,
        }
    }
}

impl PartialOrd<Self> for CamelCardHandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCardHandType {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::from(self).cmp(&u8::from(other))
    }
}

impl From<&CamelCardHand> for CamelCardHandType {
    fn from(value: &CamelCardHand) -> Self {
        let mut counts: HashMap<&char, u8> = HashMap::new();
        let mut jokers = 0u8;
        for c in value.hand.iter() {
            *counts.entry(c).or_insert(0) += 1;
            if *c == 'J' {
                jokers += 1;
            }
        }

        let mut pairs = counts.into_iter().collect::<Vec<(&char, u8)>>();
        pairs.sort_by(|(_, a), (_, b)| b.cmp(a));
        let mut iter = pairs.iter();
        while let Some((c, raw)) = iter.next() {
            if value.wildcards && **c == 'J' {
                if *raw == 5 {
                    return CamelCardHandType::FiveKind;
                }
                continue;
            }
            let count = if value.wildcards { raw + jokers } else { *raw };
            if count >= 5 {
                return CamelCardHandType::FiveKind;
            }
            if count == 4 {
                return CamelCardHandType::FourKind;
            }
            if count == 3 {
                if let Some((_, next)) = iter.next() {
                    if *next == 2 {
                        return CamelCardHandType::FullHouse;
                    }
                }
                return CamelCardHandType::ThreeKind;
            }
            if count == 2 {
                if let Some((_, next)) = iter.next() {
                    if *next == 2 {
                        return CamelCardHandType::TwoPair;
                    }
                }
                return CamelCardHandType::OnePair;
            }
            if count == 1 {
                return CamelCardHandType::HighCard;
            }
        }
        panic!("Unknown hand!")
    }
}

struct CamelCardHand {
    hand: [char; 5],
    bid: u64,
    wildcards: bool,
}

impl CamelCardHand {
    fn get_char_strength(&self, i: usize) -> u8 {
        match self.hand[i] {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => match self.wildcards {
                true => 1,
                false => 11,
            },
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Unknown card!"),
        }
    }
}

impl Eq for CamelCardHand {}

impl PartialEq<Self> for CamelCardHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand.eq(&other.hand)
    }
}

impl PartialOrd<Self> for CamelCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = CamelCardHandType::from(self);
        let other_type = CamelCardHandType::from(other);
        let type_cmp = self_type.cmp(&other_type);
        if type_cmp != Ordering::Equal {
            return type_cmp;
        }
        for i in 0usize..5 {
            let c_cmp = self.get_char_strength(i).cmp(&other.get_char_strength(i));
            if c_cmp != Ordering::Equal {
                return c_cmp;
            }
        }
        Ordering::Equal
    }
}

impl From<&str> for CamelCardHand {
    fn from(value: &str) -> Self {
        let mut hand = [' '; 5];
        let mut split = value.split(' ');
        for (i, char) in split.next().unwrap().chars().enumerate() {
            hand[i] = char;
        }
        let bid = split.next().unwrap().parse().unwrap();

        CamelCardHand {
            hand,
            bid,
            wildcards: false,
        }
    }
}

impl Display for CamelCardHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in self.hand.iter() {
            f.write_char(*c)?;
        }
        f.write_fmt(format_args!(" - {}", self.bid))?;
        Ok(())
    }
}

pub struct CamelCardHandSet {
    hands: Vec<CamelCardHand>,
}

impl CamelCardHandSet {
    pub fn to_score(&self) -> u64 {
        self.hands
            .iter()
            .enumerate()
            .map(|(i, hand)| {
                let rank = (i as u64) + 1;
                rank * hand.bid
            })
            .sum()
    }

    pub fn set_wildcards(&mut self, wildcards: bool) {
        for hand in self.hands.iter_mut() {
            hand.wildcards = wildcards;
        }
        self.hands.sort();
    }
}

impl From<Lines<'_>> for CamelCardHandSet {
    fn from(value: Lines<'_>) -> Self {
        let mut hands: Vec<CamelCardHand> = value.into_iter().map(|x| x.into()).collect();
        hands.sort();
        CamelCardHandSet { hands }
    }
}

impl Display for CamelCardHandSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for hand in self.hands.iter() {
            f.write_fmt(format_args!("{}\n", hand))?;
        }
        Ok(())
    }
}
