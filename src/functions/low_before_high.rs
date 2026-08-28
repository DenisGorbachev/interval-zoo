use Bound::{Excluded, Included, Unbounded};
use core::ops::Bound;

/// Returns whether a lower bound precedes a higher bound, treating equal values as valid only when both bounds are included.
pub fn low_before_high<T>(low: &Bound<T>, high: &Bound<T>) -> bool
where
    T: Ord,
{
    match (low, high) {
        (Unbounded, _) | (_, Unbounded) => true,
        (Included(low), Included(high)) => low <= high,
        (Included(low), Excluded(high)) | (Excluded(low), Included(high)) | (Excluded(low), Excluded(high)) => low < high,
    }
}
