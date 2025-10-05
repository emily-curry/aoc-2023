mod boat_race;

use crate::boat_race::{BoatRace, BoatRaceSeries};
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let series: BoatRaceSeries = input.to_lines().into();
    let winning_race_product = series.get_winning_race_product();
    println!("Product of winning races: {}", winning_race_product);

    let race: BoatRace = input.to_lines().into();
    let winning_race_count = race.count_winning_holds();
    println!("Count of single race wins: {}", winning_race_count);
}
