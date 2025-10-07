mod oasis;

use crate::oasis::OasisReport;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let oasis_report: OasisReport = input.to_lines().into();
    let next_values_sum = oasis_report.sum_next_values();
    println!("Sum of extrapolated next values: {}", next_values_sum);

    let prev_values_sum = oasis_report.sum_previous_values();
    println!("Sum of extrapolated previous values: {}", prev_values_sum);
}
