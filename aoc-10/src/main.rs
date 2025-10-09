mod pipe_complex;

use crate::pipe_complex::PipeComplex;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut pipe_complex: PipeComplex = input.to_lines().into();
    let midpoint = pipe_complex.get_midpoint();
    println!("Midpoint of path: {}", midpoint);

    pipe_complex.mark_outside();
    println!("Tiles inside loop: {}", pipe_complex.count_unmarked());
}
