use crate::cardinal_direction::CardinalDirection;
use crate::num::{Bounded, One};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + One<T> + Copy,
{
    pub fn go(&self, dir: &CardinalDirection) -> Self {
        match dir {
            CardinalDirection::North => Point::new(self.x, self.y - T::one()),
            CardinalDirection::South => Point::new(self.x, self.y + T::one()),
            CardinalDirection::East => Point::new(self.x + T::one(), self.y),
            CardinalDirection::West => Point::new(self.x - T::one(), self.y),
        }
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + One<T> + Copy + Bounded + PartialOrd,
{
    pub fn checked_go(&self, dir: &CardinalDirection) -> Option<Self> {
        match dir {
            CardinalDirection::North => {
                if self.y > T::MIN {
                    Some(self.go(dir))
                } else {
                    None
                }
            }
            CardinalDirection::South => {
                if self.y < T::MAX {
                    Some(self.go(dir))
                } else {
                    None
                }
            }
            CardinalDirection::East => {
                if self.x < T::MAX {
                    Some(self.go(dir))
                } else {
                    None
                }
            }
            CardinalDirection::West => {
                if self.x > T::MIN {
                    Some(self.go(dir))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cardinal_direction::CardinalDirection;
    use crate::point::Point;

    #[test]
    fn checked_go() {
        let min = Point::new(u8::MIN, u8::MIN);
        assert_eq!(min.checked_go(&CardinalDirection::West), None);
        assert_eq!(min.checked_go(&CardinalDirection::North), None);
        assert_eq!(
            min.checked_go(&CardinalDirection::South),
            Some(Point::new(0u8, 1))
        );
        assert_eq!(
            min.checked_go(&CardinalDirection::East),
            Some(Point::new(1u8, 0))
        );

        let max = Point::new(u8::MAX, u8::MAX);
        assert_eq!(
            max.checked_go(&CardinalDirection::West),
            Some(Point::new(254, 255))
        );
        assert_eq!(
            max.checked_go(&CardinalDirection::North),
            Some(Point::new(255, 254))
        );
        assert_eq!(max.checked_go(&CardinalDirection::South), None);
        assert_eq!(max.checked_go(&CardinalDirection::East), None);
    }
}
