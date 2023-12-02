mod cube_bag;

use crate::cube_bag::CubeBag;
use aoc_core::includes::Includes;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let games: Vec<CubeBag> = input.to_lines().map(CubeBag::from).collect();
    let test = CubeBag::from("Game 0: 12 red, 13 green, 14 blue");

    let sum_possible: u32 = games
        .iter()
        .filter(|c| test.includes(c))
        .map(|c| c.id)
        .sum();
    println!("Sum of ids of possible games: {}", sum_possible);

    let sum_powers: u32 = games.iter().map(CubeBag::to_power).sum();
    println!("Sum of powers of all games: {}", sum_powers);
}
