pub mod range_set;

pub trait SetDifference<T> {
    fn difference(&self, rhs: &T) -> Self;
}

pub trait SetIntersection<T> {
    fn intersection(&self, rhs: &T) -> Self;
}

pub trait SetUnion<T> {
    fn union(&self, rhs: &T) -> Self;
}
