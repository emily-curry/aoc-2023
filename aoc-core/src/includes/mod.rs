pub mod includes_range;
pub mod includes_range_inclusive;

/// Describes a thing that can be thought to fully include (or contain) another thing.
pub trait Includes<T> {
    /// Returns whether or not `self` fully contains `other`.
    fn includes(&self, other: &T) -> bool;
}
