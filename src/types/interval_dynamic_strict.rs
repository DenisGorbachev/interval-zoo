use crate::{IntervalDynamicRelaxed, Length, Overlaps, finite_bound_value, is_ordered, low_before_high};
use core::fmt::Debug;
use core::ops::{Bound, RangeBounds};
use derive_getters::Getters;
use derive_more::Into;
use errgonomic::map_err;
use num_traits::CheckedSub;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A strict interval with runtime bounds.
///
/// When both bounds are finite, the lower endpoint is guaranteed to be less than or equal to the upper endpoint. An unbounded lower endpoint represents negative infinity, while an unbounded upper endpoint represents positive infinity.
///
/// This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "IntervalDynamicStrictInput<T>", bound(deserialize = "T: Debug + Deserialize<'de> + Ord")))]
#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalDynamicStrict<T> {
    lo: Bound<T>,
    hi: Bound<T>,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct IntervalDynamicStrictInput<T> {
    lo: Bound<T>,
    hi: Bound<T>,
}

impl<T> IntervalDynamicStrict<T> {
    pub fn map<U>(self, mut f: impl FnMut(Bound<T>) -> Bound<U>) -> Result<IntervalDynamicStrict<U>, IntervalDynamicStrictMapError<U>>
    where
        U: Ord + Debug,
    {
        use IntervalDynamicStrictMapError::*;
        let lo = f(self.lo);
        let hi = f(self.hi);
        map_err!(IntervalDynamicStrict::try_from((lo, hi)), TryFromFailed)
    }

    /// Maps both bounds without checking the output interval's order.
    ///
    /// # Safety
    ///
    /// `f` must preserve the `lo <= hi` invariant.
    pub unsafe fn map_unchecked<U>(self, mut f: impl FnMut(Bound<T>) -> Bound<U>) -> IntervalDynamicStrict<U> {
        IntervalDynamicStrict {
            lo: f(self.lo),
            hi: f(self.hi),
        }
    }
}

impl<T> IntervalDynamicStrict<T>
where
    T: Ord,
{
    pub fn new_ordered(lo: impl Into<Bound<T>>, hi: impl Into<Bound<T>>) -> Self {
        let lo = lo.into();
        let hi = hi.into();
        let order_check = is_ordered(&lo, &hi);
        match (order_check, lo, hi) {
            (true, lo, hi) => Self {
                lo,
                hi,
            },
            (false, lo, hi) => Self {
                lo: hi,
                hi: lo,
            },
        }
    }
}

impl<T> TryFrom<(Bound<T>, Bound<T>)> for IntervalDynamicStrict<T>
where
    T: Ord + Debug,
{
    type Error = TryFromTupleForIntervalDynamicStrictError<T>;

    fn try_from((lo, hi): (Bound<T>, Bound<T>)) -> Result<Self, Self::Error> {
        use TryFromTupleForIntervalDynamicStrictError::*;
        let order_check = is_ordered(&lo, &hi);
        match (order_check, lo, hi) {
            (true, lo, hi) => Ok(Self {
                lo,
                hi,
            }),
            (false, lo, hi) => Err(OrderCheckFailed {
                lo,
                hi,
            }),
        }
    }
}

impl<T> TryFrom<IntervalDynamicRelaxed<T>> for IntervalDynamicStrict<T>
where
    T: Ord + Debug,
{
    type Error = TryFromIntervalDynamicRelaxedForIntervalDynamicStrictError<T>;

    fn try_from(value: IntervalDynamicRelaxed<T>) -> Result<Self, Self::Error> {
        use TryFromIntervalDynamicRelaxedForIntervalDynamicStrictError::*;
        let IntervalDynamicRelaxed {
            a: lo,
            b: hi,
        } = value;
        let order_check = is_ordered(&lo, &hi);
        match (order_check, lo, hi) {
            (true, lo, hi) => Ok(Self {
                lo,
                hi,
            }),
            (false, lo, hi) => Err(OrderCheckFailed {
                a: lo,
                b: hi,
            }),
        }
    }
}

#[cfg(feature = "serde")]
impl<T> TryFrom<IntervalDynamicStrictInput<T>> for IntervalDynamicStrict<T>
where
    T: Ord + Debug,
{
    type Error = TryFromIntervalDynamicStrictInputForIntervalDynamicStrictError<T>;

    fn try_from(value: IntervalDynamicStrictInput<T>) -> Result<Self, Self::Error> {
        use TryFromIntervalDynamicStrictInputForIntervalDynamicStrictError::*;
        let IntervalDynamicStrictInput {
            lo,
            hi,
        } = value;
        let order_check = is_ordered(&lo, &hi);
        match (order_check, lo, hi) {
            (true, lo, hi) => Ok(Self {
                lo,
                hi,
            }),
            (false, lo, hi) => Err(OrderCheckFailed {
                lo,
                hi,
            }),
        }
    }
}

impl<T> Length for IntervalDynamicStrict<T>
where
    T: CheckedSub,
{
    type Output = Option<T>;

    fn length(&self) -> Self::Output {
        match (finite_bound_value(&self.lo), finite_bound_value(&self.hi)) {
            (Some(lo), Some(hi)) => hi.checked_sub(lo),
            _ => None,
        }
    }
}

impl<T> RangeBounds<T> for IntervalDynamicStrict<T> {
    fn start_bound(&self) -> Bound<&T> {
        self.lo.as_ref()
    }

    fn end_bound(&self) -> Bound<&T> {
        self.hi.as_ref()
    }
}

impl<T> Overlaps<Self> for IntervalDynamicStrict<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        low_before_high(&self.lo, &self.hi) && low_before_high(&other.lo, &other.hi) && low_before_high(&self.lo, &other.hi) && low_before_high(&other.lo, &self.hi)
    }
}

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromTupleForIntervalDynamicStrictError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { lo: Bound<T>, hi: Bound<T> },
}

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromIntervalDynamicRelaxedForIntervalDynamicStrictError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { a: Bound<T>, b: Bound<T> },
}

#[cfg(feature = "serde")]
#[derive(Error, Clone, Copy, Debug)]
enum TryFromIntervalDynamicStrictInputForIntervalDynamicStrictError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { lo: Bound<T>, hi: Bound<T> },
}

#[derive(Error, Clone, Copy, Debug)]
pub enum IntervalDynamicStrictMapError<T>
where
    T: Debug,
{
    #[error("failed to map strict interval bounds")]
    TryFromFailed { source: TryFromTupleForIntervalDynamicStrictError<T> },
}
