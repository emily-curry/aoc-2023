mod seed_map;

use crate::seed_map::SeedAlmanac;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let almanac: SeedAlmanac = input.as_str().into();
    let location_individual = almanac.to_location_individual();
    println!(
        "Min location number as individual seeds: {}",
        location_individual.iter().min().unwrap()
    );

    let location_range = almanac.to_location_range_min();
    println!("Min location number as ranges of seeds: {}", location_range);
}
