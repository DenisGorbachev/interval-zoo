use crate::{IntervalDynamicStrict, Length, Overlaps, finite_bound_value, is_ordered, low_before_high};
use core::cmp::Ordering::Greater;
use core::fmt::Debug;
use core::mem::swap;
use derive_getters::{Dissolve, Getters};
use derive_more::From;
use derive_new::new;
use num_traits::CheckedSub;
use std::ops::{Bound, Range, RangeBounds};
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use Bound::{Excluded, Included};

/// A relaxed interval with runtime bounds.
///
/// Prefer [`IntervalDynamicStrict`] that implements validation.
///
/// This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        if !is_ordered(&self.a, &self.b) {
            swap(&mut self.a, &mut self.b);
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

impl<T> From<IntervalDynamicStrict<T>> for IntervalDynamicRelaxed<T> {
    fn from(value: IntervalDynamicStrict<T>) -> Self {
        let (a, b): (Bound<T>, Bound<T>) = value.into();
        Self::new(a, b)
    }
}

impl<T> TryFrom<IntervalDynamicRelaxed<T>> for Range<T>
where
    T: Debug,
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

impl<T> RangeBounds<T> for IntervalDynamicRelaxed<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.a.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.b.as_ref()
    }
}

impl<T> Overlaps<Self> for IntervalDynamicRelaxed<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        low_before_high(&self.a, &self.b) && low_before_high(&other.a, &other.b) && low_before_high(&self.a, &other.b) && low_before_high(&other.a, &self.b)
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

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromIntervalDynamicRelaxedForRangeError<T> {
    #[error("failed to convert dynamic relaxed interval into range")]
    ConversionFailed { a: Bound<T>, b: Bound<T> },
}
