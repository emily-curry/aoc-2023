mod calibration_value;

use crate::calibration_value::CalibrationValue;
use aoc_core::puzzle_input::PuzzleInput;

fn main() {
    let input = PuzzleInput::default();
    let mut values: Vec<CalibrationValue> = input.to_lines().map(CalibrationValue::from).collect();

    let sum1: u32 = values.iter().map(|cv| cv.to_u32()).sum();
    println!("Sum of all calibration values using only digits: {}", sum1);

    values.iter_mut().for_each(|cv| cv.replace_words());
    let sum2: u32 = values.iter().map(|cv| cv.to_u32()).sum();
    println!(
        "Sum of all calibration values using digits and words: {}",
        sum2
    );
}
