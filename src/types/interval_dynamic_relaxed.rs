use crate::{Contains, Length, Overlaps};
use core::cmp::Ordering::Greater;
use derive_getters::{Dissolve, Getters};
use derive_more::From;
use derive_new::new;
use num_traits::CheckedSub;
use std::ops::{Bound, Range};
use thiserror::Error;

use Bound::{Excluded, Included, Unbounded};

/// A relaxed interval with runtime bounds.
///
/// This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.
#[derive(new, Getters, Dissolve, From, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalDynamicRelaxed<T> {
    pub a: Bound<T>,
    pub b: Bound<T>,
}

impl<T> IntervalDynamicRelaxed<T> {
    pub fn new_inclusive_exclusive(a: T, b: T) -> Self {
        Self::new(Included(a), Excluded(b))
    }
}

impl<T> IntervalDynamicRelaxed<T>
where
    T: Ord,
{
    pub fn new_ordered(a: Bound<T>, b: Bound<T>) -> Self {
        let mut interval = Self::new(a, b);
        interval.normalize();
        interval
    }

    pub fn normalize(&mut self) {
        if bounds_are_reversed(&self.a, &self.b) {
            core::mem::swap(&mut self.a, &mut self.b);
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        Overlaps::overlaps(self, other)
    }
}

impl<T> From<Range<T>> for IntervalDynamicRelaxed<T> {
    fn from(value: Range<T>) -> Self {
        Self::new(Included(value.start), Excluded(value.end))
    }
}

impl<T> TryFrom<IntervalDynamicRelaxed<T>> for Range<T>
where
    T: core::fmt::Debug,
{
    type Error = TryFromIntervalDynamicRelaxedForRangeError<T>;

    fn try_from(value: IntervalDynamicRelaxed<T>) -> Result<Self, Self::Error> {
        use TryFromIntervalDynamicRelaxedForRangeError::*;
        let IntervalDynamicRelaxed {
            a,
            b,
        } = value;
        match (a, b) {
            (Included(start), Excluded(end)) => Ok(Self {
                start,
                end,
            }),
            (a, b) => Err(ConversionFailed {
                a,
                b,
            }),
        }
    }
}

impl<T> Contains<T> for IntervalDynamicRelaxed<T>
where
    T: Ord,
{
    fn contains(&self, value: &T) -> bool {
        lower_contains(&self.a, value) && upper_contains(&self.b, value)
    }
}

impl<T> Overlaps<Self> for IntervalDynamicRelaxed<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        lower_before_upper(&self.a, &other.b) && lower_before_upper(&other.a, &self.b)
    }
}

impl<T> Length for IntervalDynamicRelaxed<T>
where
    T: CheckedSub + Ord,
{
    type Output = Option<T>;

    fn length(&self) -> Self::Output {
        match (finite_bound_value(&self.a), finite_bound_value(&self.b)) {
            (Some(a), Some(b)) => match a.cmp(b) {
                Greater => a.checked_sub(b),
                _ => b.checked_sub(a),
            },
            _ => None,
        }
    }
}

fn bounds_are_reversed<T>(lower: &Bound<T>, upper: &Bound<T>) -> bool
where
    T: Ord,
{
    match (lower, upper) {
        (Included(lower) | Excluded(lower), Included(upper) | Excluded(upper)) => lower.cmp(upper).is_gt(),
        (Unbounded, _) | (_, Unbounded) => false,
    }
}

fn lower_contains<T>(lower: &Bound<T>, value: &T) -> bool
where
    T: Ord,
{
    match lower {
        Included(lower) => lower <= value,
        Excluded(lower) => lower < value,
        Unbounded => true,
    }
}

fn upper_contains<T>(upper: &Bound<T>, value: &T) -> bool
where
    T: Ord,
{
    match upper {
        Included(upper) => value <= upper,
        Excluded(upper) => value < upper,
        Unbounded => true,
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

fn finite_bound_value<T>(bound: &Bound<T>) -> Option<&T> {
    match bound {
        Included(value) | Excluded(value) => Some(value),
        Unbounded => None,
    }
}

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromIntervalDynamicRelaxedForRangeError<T> {
    #[error("failed to convert dynamic relaxed interval into range")]
    ConversionFailed { a: Bound<T>, b: Bound<T> },
}
