mod scratchcard;

use crate::scratchcard::{Scratchcard, ScratchcardStack};
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let scratchcards: ScratchcardStack = input.to_lines().into();
    let point_sum = scratchcards.to_point_sum();
    println!("Sum of scratchcard point values: {}", point_sum);
    let copies_sum = scratchcards.to_copies_sum();
    println!("Sum of scratchcard copies: {}", copies_sum);
}
