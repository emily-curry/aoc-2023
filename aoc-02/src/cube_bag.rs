use aoc_core::includes::Includes;
use std::cmp::max;

pub struct CubeBag {
    pub id: u32,
    green: u8,
    red: u8,
    blue: u8,
}

impl CubeBag {
    pub fn to_power(&self) -> u32 {
        (self.green as u32) * (self.red as u32) * (self.blue as u32)
    }
}

impl Includes<CubeBag> for CubeBag {
    fn includes(&self, other: &CubeBag) -> bool {
        self.green >= other.green && self.blue >= other.blue && self.red >= other.red
    }
}

impl From<&str> for CubeBag {
    fn from(value: &str) -> Self {
        let mut outer_iter = value.split(": ");
        let id: u32 = outer_iter
            .next()
            .unwrap()
            .split(" ")
            .skip(1)
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let mut red = 0u8;
        let mut green = 0u8;
        let mut blue = 0u8;

        let grabs = outer_iter.next().unwrap().split("; ");
        for grab in grabs {
            for mut color_count in grab.split(", ").map(|i| i.split(" ")) {
                let count: u8 = color_count.next().unwrap().parse().unwrap();
                let color = color_count.next().unwrap();
                match color {
                    "green" => green = max(green, count),
                    "red" => red = max(red, count),
                    "blue" => blue = max(blue, count),
                    _ => panic!("Unknown color"),
                }
            }
        }

        CubeBag {
            id,
            red,
            green,
            blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cube_bag::CubeBag;

    #[test]
    fn from_str() {
        let cv = CubeBag::from("Game 84: 16 red, 2 green, 6 blue; 6 red, 3 green, 8 blue; 3 green, 10 red, 5 blue; 4 blue, 3 green; 15 red");
        assert_eq!(cv.id, 84);
        assert_eq!(cv.red, 16);
        assert_eq!(cv.green, 3);
        assert_eq!(cv.blue, 8);
    }
}
