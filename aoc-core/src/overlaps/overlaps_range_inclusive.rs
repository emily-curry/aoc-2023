use crate::overlaps::Overlaps;
use std::ops::RangeInclusive;

fn starts_in<T>(s: &RangeInclusive<T>, o: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    s.start() >= o.start() && s.start() <= o.end()
}

fn ends_in<T>(s: &RangeInclusive<T>, o: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    s.end() >= o.start() && s.end() <= o.end()
}

impl<T> Overlaps<RangeInclusive<T>> for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &RangeInclusive<T>) -> bool {
        starts_in(self, other)
            || ends_in(self, other)
            || starts_in(other, self)
            || ends_in(other, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::overlaps::Overlaps;

    #[test]
    fn does_detect_overlap() {
        assert_eq!((0..=5).overlaps(&(1..=7)), true);
        assert_eq!((0..=5).overlaps(&(7..=10)), false);
        assert_eq!((0..=5).overlaps(&(2..=3)), true);
    }

    #[test]
    fn is_inclusive_on_upper_bound() {
        assert_eq!((10..=15).overlaps(&(15..=20)), true);
    }

    #[test]
    fn order_does_not_matter() {
        assert_eq!((0..=5).overlaps(&(1..=7)), (1..=7).overlaps(&(0..=5)));
        assert_eq!((0..=5).overlaps(&(7..=10)), (7..=10).overlaps(&(0..=5)));
        assert_eq!((0..=5).overlaps(&(2..=3)), (2..=3).overlaps(&(0..=5)));
        assert_eq!(
            (10..=15).overlaps(&(15..=20)),
            (15..=20).overlaps(&(10..=15))
        );
    }
}
