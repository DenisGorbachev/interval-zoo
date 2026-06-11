use std::ops::{Bound, Range};

use derive_getters::{Dissolve, Getters};
use derive_more::From;
use derive_new::new;

use Bound::{Excluded, Included, Unbounded};

#[derive(new, Getters, Dissolve, From, Eq, PartialEq, Hash, Clone, Debug)]
pub struct IntervalDynamicRelaxed<T> {
    pub a: Bound<T>,
    pub b: Bound<T>,
}

impl<T> IntervalDynamicRelaxed<T> {
    pub fn new_inclusive_exclusive(a: T, b: T) -> Self {
        Self::new(Included(a), Excluded(b))
    }
}

impl<T> From<Range<T>> for IntervalDynamicRelaxed<T> {
    fn from(value: Range<T>) -> Self {
        Self::new(Included(value.start), Excluded(value.end))
    }
}

impl<T> From<IntervalDynamicRelaxed<T>> for Range<T> {
    fn from(value: IntervalDynamicRelaxed<T>) -> Self {
        let start = match value.a {
            Included(start) => start,
            Excluded(_) => panic!("Cannot convert interval with an excluded start bound into Range"),
            Unbounded => panic!("Cannot convert interval with an unbounded start into Range"),
        };
        let end = match value.b {
            Excluded(end) => end,
            Included(_) => panic!("Cannot convert interval with an included end bound into Range"),
            Unbounded => panic!("Cannot convert interval with an unbounded end into Range"),
        };
        start..end
    }
}

impl<T> IntervalDynamicRelaxed<T>
where
    T: Ord,
{
    pub fn overlaps(&self, other: &IntervalDynamicRelaxed<T>) -> bool {
        lower_before_upper(&self.a, &other.b) && lower_before_upper(&other.a, &self.b)
    }
}

fn lower_before_upper<T>(lower: &Bound<T>, upper: &Bound<T>) -> bool
where
    T: Ord,
{
    match (lower, upper) {
        (Unbounded, _) | (_, Unbounded) => true,
        (Included(lower), Included(upper)) => lower <= upper,
        (Included(lower), Excluded(upper)) | (Excluded(lower), Included(upper)) | (Excluded(lower), Excluded(upper)) => lower < upper,
    }
}
