use Bound::{Excluded, Included, Unbounded};
use core::ops::Bound;

/// Returns whether a lower and upper bound are ordered.
pub fn is_ordered<T>(low: &Bound<T>, high: &Bound<T>) -> bool
where
    T: Ord,
{
    match (low, high) {
        (Included(low) | Excluded(low), Included(high) | Excluded(high)) => low <= high,
        (Unbounded, _) | (_, Unbounded) => true,
    }
}
