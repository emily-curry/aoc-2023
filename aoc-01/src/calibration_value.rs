use std::fmt::{Display, Formatter, Write};

#[derive(Debug)]
pub struct CalibrationValue {
    raw: Vec<char>,
}

impl CalibrationValue {
    pub fn to_u32(&self) -> u32 {
        let mut iter = self.raw.iter().filter(|c| c.is_ascii_digit());
        let first = iter.next().unwrap().to_digit(10).unwrap();
        let last = iter
            .last()
            .map(|c| c.to_digit(10).unwrap())
            .unwrap_or(first);
        (first * 10) + last
    }

    pub fn replace_words(&mut self) -> () {
        let str: String = self.raw.iter().collect();
        // words in the input may share contiguous characters with other words (e.g. "oneight")
        let res = str
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e");
        self.raw = res.chars().collect();
    }
}

impl From<&str> for CalibrationValue {
    fn from(value: &str) -> Self {
        CalibrationValue {
            raw: value.chars().collect(),
        }
    }
}

impl Into<u32> for CalibrationValue {
    fn into(self) -> u32 {
        self.to_u32()
    }
}

impl Display for CalibrationValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for c in &self.raw {
            f.write_char(*c)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::calibration_value::CalibrationValue;

    #[test]
    fn to_u32() {
        let cv = CalibrationValue::from("abc3ve2a1l");
        assert_eq!(cv.to_u32(), 31);

        let cv = CalibrationValue::from("123");
        assert_eq!(cv.to_u32(), 13);

        let cv = CalibrationValue::from("avl8sed");
        assert_eq!(cv.to_u32(), 88);
    }

    #[test]
    fn replace_words() {
        let mut cv = CalibrationValue::from("onetwothreefourfivesixseveneightnine");
        cv.replace_words();
        assert_eq!(cv.to_u32(), 19);
    }
}
