mod schematic;

use crate::schematic::Schematic;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let schematic: Schematic = input.to_lines().into();
    // println!("Full schematic:\n{}", schematic);
    let part_numbers = schematic.to_part_numbers();
    let part_sum: u32 = part_numbers.iter().sum();
    println!("Sum of all part numbers: {}", part_sum);
    let gear_ratios = schematic.to_gear_ratios();
    let gear_sum: u32 = gear_ratios.iter().sum();
    println!("Sum of all gear ratios: {}", gear_sum);
}
