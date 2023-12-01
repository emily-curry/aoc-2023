use crate::overlaps::Overlaps;
use std::ops::Range;

fn starts_in<T>(s: &Range<T>, o: &Range<T>) -> bool
where
    T: PartialOrd,
{
    s.start >= o.start && s.start < o.end
}

fn ends_in<T>(s: &Range<T>, o: &Range<T>) -> bool
where
    T: PartialOrd,
{
    s.end > o.start && s.end <= o.end
}

impl<T> Overlaps<Range<T>> for Range<T>
where
    T: PartialOrd,
{
    fn overlaps(&self, other: &Range<T>) -> bool {
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
        assert_eq!((0..5).overlaps(&(1..7)), true);
        assert_eq!((0..5).overlaps(&(7..10)), false);
        assert_eq!((0..5).overlaps(&(2..3)), true);
    }

    #[test]
    fn is_exclusive_on_upper_bound() {
        assert_eq!((10..15).overlaps(&(15..20)), false);
    }

    #[test]
    fn order_does_not_matter() {
        assert_eq!((0..5).overlaps(&(1..7)), (1..7).overlaps(&(0..5)));
        assert_eq!((0..5).overlaps(&(7..10)), (7..10).overlaps(&(0..5)));
        assert_eq!((0..5).overlaps(&(2..3)), (2..3).overlaps(&(0..5)));
        assert_eq!((10..15).overlaps(&(15..20)), (15..20).overlaps(&(10..15)));
    }
}
