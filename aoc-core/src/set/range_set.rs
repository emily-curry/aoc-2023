use crate::includes::Includes;
use crate::overlaps::Overlaps;
use crate::set::{SetDifference, SetIntersection, SetUnion};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::slice::Iter;

pub struct RangeSet<T> {
    ranges: Vec<Range<T>>,
}

impl<T> RangeSet<T>
where
    T: PartialOrd + Ord + Clone,
{
    pub fn new(ranges: Vec<Range<T>>) -> Self {
        let mut r = RangeSet { ranges };
        r.cleanup();
        r
    }

    pub fn iter(&self) -> Iter<'_, Range<T>> {
        self.ranges.iter()
    }

    fn cleanup(&mut self) {
        self.sort();
        // if there's nothing to consolidate, return.
        if self.ranges.len() <= 1 {
            return;
        }

        let mut result: Vec<Range<T>> = vec![];
        let mut current = self.ranges.first().unwrap().clone();
        for step in self.ranges.iter().skip(1) {
            if current.end <= step.start {
                result.push(current);
                current = step.clone();
            } else {
                current.end = current.end.max(step.end.clone());
            }
        }
        result.push(current);
        self.ranges = result
    }

    fn sort(&mut self) {
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
    }
}

impl<T> SetDifference<Self> for RangeSet<T>
where
    T: PartialOrd + Ord + Clone + Display,
{
    fn difference(&self, rhs: &Self) -> Self {
        let result = rhs
            .ranges
            .iter()
            .fold(self.ranges.clone(), |acc, rhs_range| {
                let mut step_result: Vec<Range<T>> = vec![];
                for lhs_range in acc {
                    // if no overlap at all, the entire lhs range remains in the range set
                    if !lhs_range.overlaps(rhs_range) {
                        step_result.push(lhs_range);
                    } else {
                        // otherwise, keep only the ranges not in rhs
                        if lhs_range.start < rhs_range.start {
                            step_result.push(lhs_range.start..rhs_range.start.clone());
                        }
                        if lhs_range.end > rhs_range.end {
                            step_result.push(rhs_range.end.clone()..lhs_range.end);
                        }
                    }
                }
                step_result
            });

        RangeSet::new(result)
    }
}

impl<T> SetIntersection<Self> for RangeSet<T>
where
    T: PartialOrd + Ord + Clone,
{
    fn intersection(&self, rhs: &Self) -> Self {
        let mut result = vec![];
        for lhs_range in self.ranges.iter() {
            for rhs_range in rhs.ranges.iter() {
                if lhs_range.start < rhs_range.end && lhs_range.end > rhs_range.start {
                    let start = lhs_range.start.clone().max(rhs_range.start.clone());
                    let end = lhs_range.end.clone().min(rhs_range.end.clone());
                    result.push(start..end);
                }
            }
        }

        RangeSet::new(result)
    }
}

impl<T> SetUnion<Self> for RangeSet<T>
where
    T: PartialOrd + Ord + Clone,
{
    fn union(&self, rhs: &Self) -> Self {
        let mut result = self.ranges.clone();
        result.append(&mut rhs.ranges.clone());
        RangeSet::new(result)
    }
}

impl<T> Includes<T> for RangeSet<T>
where
    T: PartialOrd,
{
    fn includes(&self, other: &T) -> bool {
        self.ranges.iter().any(|r| r.contains(other))
    }
}

impl<T> Display for RangeSet<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RangeSet[")?;
        for (i, r) in self.ranges.iter().enumerate() {
            f.write_fmt(format_args!("{}..{}", r.start, r.end))?;
            if i != self.ranges.len() - 1 {
                f.write_str(", ")?;
            }
        }
        f.write_str("]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::includes::Includes;
    use crate::set::range_set::RangeSet;
    use crate::set::{SetDifference, SetIntersection, SetUnion};

    #[test]
    fn intersection() {
        let set_a = RangeSet::new(vec![2..5, 6..9]);
        let set_b = RangeSet::new(vec![3..7]);
        let set_c = set_a.intersection(&set_b);
        let set_d = set_b.intersection(&set_a);
        let expected = vec![3, 4, 6];
        for i in 0..10 {
            let expect = if expected.contains(&i) { true } else { false };
            assert_eq!(set_c.includes(&i), expect);
            assert_eq!(set_d.includes(&i), expect);
        }
    }

    #[test]
    fn difference() {
        let set_a = RangeSet::new(vec![2..5, 6..9, 13..15]);
        let set_b = RangeSet::new(vec![3..7, 12..20]);
        let set_c = set_a.difference(&set_b);
        let expected = vec![2, 7, 8];
        for i in 0..20 {
            let expect = if expected.contains(&i) { true } else { false };
            assert_eq!(set_c.includes(&i), expect);
        }
        let set_d = set_b.difference(&set_a);
        let expected = vec![5, 12, 15, 16, 17, 18, 19];
        for i in 0..20 {
            let expect = if expected.contains(&i) { true } else { false };
            assert_eq!(set_d.includes(&i), expect);
        }
    }

    #[test]
    fn union() {
        let set_a = RangeSet::new(vec![2..5, 6..9]);
        let set_b = RangeSet::new(vec![3..7]);
        let set_c = set_a.union(&set_b);
        let set_d = set_b.union(&set_a);
        let expected = vec![2, 3, 4, 5, 6, 7, 8];
        for i in 0..10 {
            let expect = if expected.contains(&i) { true } else { false };
            assert_eq!(set_c.includes(&i), expect);
            assert_eq!(set_d.includes(&i), expect);
        }
    }
}
