extern crate core;

pub mod cardinal_direction;
pub mod includes;
pub mod num;
pub mod overlaps;
pub mod point;
pub mod puzzle_input;
pub mod set;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
