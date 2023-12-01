use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::str::Lines;

#[derive(Debug)]
pub struct PuzzleInput {
    raw: String,
}

impl PuzzleInput {
    /// Reads the file provided in `path` and returns something usable by the puzzles.
    /// This panics immediately if the file cannot be read.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let raw_result = read_to_string(path);
        PuzzleInput {
            raw: raw_result.unwrap(),
        }
    }

    /// Reads the file named "input.txt" in the directory of the currently executing crate.
    // pub fn new() -> Self {
    //     let basePath = std::env::var("CARGO_PKG_NAME").unwrap();
    //     for r in std::env::vars() {
    //         println!("{:?}", r);
    //     }
    //     panic!()
    // }

    pub fn as_string(&self) -> &String {
        &self.raw
    }

    pub fn to_lines(&self) -> Lines {
        self.raw.lines()
    }
}

impl Default for PuzzleInput {
    fn default() -> Self {
        let base_path = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let path_buf: PathBuf = [&base_path, "input.txt"].iter().collect();
        PuzzleInput::new(path_buf)
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzle_input::PuzzleInput;
    use std::ops::Index;

    #[test]
    fn can_construct_without_panic() {
        let input = PuzzleInput::new("./input.txt");
        assert!(!input.raw.is_empty());
    }

    #[test]
    fn as_string() {
        let input = PuzzleInput::new("./input.txt");
        assert!(!input.as_string().is_empty());
    }

    #[test]
    fn to_lines() {
        let input = PuzzleInput::new("./input.txt");
        let lines = input.to_lines().collect::<Vec<&str>>();
        assert_eq!(input.to_lines().collect::<Vec<&str>>().len(), 25);
        assert_eq!(*lines.index(3), "4635")
    }

    #[test]
    fn can_autodetect_file() {
        let input = PuzzleInput::default();
        let lines = input.to_lines();
        assert_eq!(lines.skip(3).next().unwrap(), "4635");
    }
}
