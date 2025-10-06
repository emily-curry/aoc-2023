mod desert_map;

use crate::desert_map::DesertMap;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let map: DesertMap = input.to_lines().into();
    let steps = map.navigate();
    println!("Steps to get from AAA to ZZZ as human: {}", steps);

    let steps = map.navigate_ghost();
    println!("Steps to get from ??A to ??Z as human: {}", steps);
}
