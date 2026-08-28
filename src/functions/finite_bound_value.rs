use Bound::{Excluded, Included, Unbounded};
use core::ops::Bound;

/// Returns the value of a finite bound, or [`None`] for [`Unbounded`].
pub fn finite_bound_value<T>(bound: &Bound<T>) -> Option<&T> {
    match bound {
        Included(value) | Excluded(value) => Some(value),
        Unbounded => None,
    }
}
