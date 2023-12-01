pub mod overlaps_range;
pub mod overlaps_range_inclusive;

/// Describes a thing that can be thought to partially overlap with another thing.
pub trait Overlaps<T> {
    /// Returns whether or not `self` at least partially overlaps with `other`.
    fn overlaps(&self, other: &T) -> bool;
}
