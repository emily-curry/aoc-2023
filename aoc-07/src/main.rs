mod camel_card;

use crate::camel_card::CamelCardHandSet;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut hand_set: CamelCardHandSet = input.to_lines().into();
    let score = hand_set.to_score();
    println!("Sum of all ranked scores: {}", score);

    hand_set.set_wildcards(true);
    let score = hand_set.to_score();
    println!("Sum of all ranked scores with wildcard rule: {}", score);
    // println!("{}", hand_set)
}
