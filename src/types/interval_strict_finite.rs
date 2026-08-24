use crate::{Length, Overlaps};
use core::fmt::Debug;
use core::ops::Bound::*;
use core::ops::{Bound, RangeBounds};
use derive_getters::Getters;
use derive_more::Into;
use errgonomic::map_err;
use num_traits::CheckedSub;
use thiserror::Error;

pub const EXCLUDED: bool = false;
pub const INCLUDED: bool = true;

/// A strict finite interval.
///
/// This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.
#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalStrictFinite<T, const LO_INC: bool, const HI_INC: bool> {
    lo: T,
    hi: T,
}

pub type IntervalStrictFiniteExcExc<T> = IntervalStrictFinite<T, EXCLUDED, EXCLUDED>;
pub type IntervalStrictFiniteExcInc<T> = IntervalStrictFinite<T, EXCLUDED, INCLUDED>;
pub type IntervalStrictFiniteIncExc<T> = IntervalStrictFinite<T, INCLUDED, EXCLUDED>;
pub type IntervalStrictFiniteIncInc<T> = IntervalStrictFinite<T, INCLUDED, INCLUDED>;

impl<T, const A_INC: bool, const B_INC: bool> IntervalStrictFinite<T, A_INC, B_INC> {
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Result<IntervalStrictFinite<U, A_INC, B_INC>, IntervalStrictFiniteMapError<U>>
    where
        U: Ord + Debug,
    {
        use IntervalStrictFiniteMapError::*;
        let lo = f(self.lo);
        let hi = f(self.hi);
        map_err!(IntervalStrictFinite::try_from((lo, hi)), TryFromFailed)
    }

    /// Maps both endpoints without checking the output interval's order.
    ///
    /// # Safety
    ///
    /// `f` must preserve the `lo <= hi` invariant.
    pub unsafe fn map_unchecked<U>(self, mut f: impl FnMut(T) -> U) -> IntervalStrictFinite<U, A_INC, B_INC> {
        IntervalStrictFinite {
            lo: f(self.lo),
            hi: f(self.hi),
        }
    }
}

impl<T, const LO_INC: bool, const HI_INC: bool> IntervalStrictFinite<T, LO_INC, HI_INC>
where
    T: Ord,
{
    pub fn new_ordered(lo: impl Into<T>, hi: impl Into<T>) -> Self {
        let lo = lo.into();
        let hi = hi.into();
        use core::cmp::Ordering::*;
        match lo.cmp(&hi) {
            Greater => Self {
                lo: hi,
                hi: lo,
            },
            Equal | Less => Self {
                lo,
                hi,
            },
        }
    }
}

impl<T, const LO_INC: bool, const HI_INC: bool> TryFrom<(T, T)> for IntervalStrictFinite<T, LO_INC, HI_INC>
where
    T: Ord + Debug,
{
    type Error = TryFromTupleForIntervalStrictFiniteError<T>;

    fn try_from((lo, hi): (T, T)) -> Result<Self, Self::Error> {
        use TryFromTupleForIntervalStrictFiniteError::*;
        use core::cmp::Ordering::*;
        let order = lo.cmp(&hi);
        match (order, lo, hi) {
            (Equal | Less, lo, hi) => Ok(Self {
                lo,
                hi,
            }),
            (Greater, lo, hi) => Err(OrderCheckFailed {
                lo,
                hi,
            }),
        }
    }
}

impl<T, const LO_INC: bool, const HI_INC: bool> Length for IntervalStrictFinite<T, LO_INC, HI_INC>
where
    T: CheckedSub,
{
    type Output = Option<T>;

    fn length(&self) -> Self::Output {
        self.hi.checked_sub(&self.lo)
    }
}

impl<T, const LO_INC: bool, const HI_INC: bool> RangeBounds<T> for IntervalStrictFinite<T, LO_INC, HI_INC> {
    fn start_bound(&self) -> Bound<&T> {
        if LO_INC { Included(&self.lo) } else { Excluded(&self.lo) }
    }

    fn end_bound(&self) -> Bound<&T> {
        if HI_INC { Included(&self.hi) } else { Excluded(&self.hi) }
    }
}

macro_rules! impl_overlaps {
    ($lo_inc:expr, $hi_inc:expr, $other_lo_inc:expr, $other_hi_inc:expr, $left_op:tt, $right_op:tt) => {
        impl<T> Overlaps<IntervalStrictFinite<T, $other_lo_inc, $other_hi_inc>> for IntervalStrictFinite<T, $lo_inc, $hi_inc>
        where
            T: Ord,
        {
            fn overlaps(&self, other: &IntervalStrictFinite<T, $other_lo_inc, $other_hi_inc>) -> bool {
                self.lo $left_op other.hi && other.lo $right_op self.hi
            }
        }
    };
}

impl_overlaps!(EXCLUDED, EXCLUDED, EXCLUDED, EXCLUDED, <, <);
impl_overlaps!(EXCLUDED, EXCLUDED, EXCLUDED, INCLUDED, <, <);
impl_overlaps!(EXCLUDED, EXCLUDED, INCLUDED, EXCLUDED, <, <);
impl_overlaps!(EXCLUDED, EXCLUDED, INCLUDED, INCLUDED, <, <);
impl_overlaps!(EXCLUDED, INCLUDED, EXCLUDED, EXCLUDED, <, <);
impl_overlaps!(EXCLUDED, INCLUDED, EXCLUDED, INCLUDED, <, <);
impl_overlaps!(EXCLUDED, INCLUDED, INCLUDED, EXCLUDED, <, <=);
impl_overlaps!(EXCLUDED, INCLUDED, INCLUDED, INCLUDED, <, <=);
impl_overlaps!(INCLUDED, EXCLUDED, EXCLUDED, EXCLUDED, <, <);
impl_overlaps!(INCLUDED, EXCLUDED, EXCLUDED, INCLUDED, <=, <);
impl_overlaps!(INCLUDED, EXCLUDED, INCLUDED, EXCLUDED, <, <);
impl_overlaps!(INCLUDED, EXCLUDED, INCLUDED, INCLUDED, <=, <);
impl_overlaps!(INCLUDED, INCLUDED, EXCLUDED, EXCLUDED, <, <);
impl_overlaps!(INCLUDED, INCLUDED, EXCLUDED, INCLUDED, <=, <);
impl_overlaps!(INCLUDED, INCLUDED, INCLUDED, EXCLUDED, <, <=);
impl_overlaps!(INCLUDED, INCLUDED, INCLUDED, INCLUDED, <=, <=);

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromTupleForIntervalStrictFiniteError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { lo: T, hi: T },
}

#[derive(Error, Clone, Copy, Debug)]
pub enum IntervalStrictFiniteMapError<T>
where
    T: Debug,
{
    #[error("failed to map strict interval bounds")]
    TryFromFailed { source: TryFromTupleForIntervalStrictFiniteError<T> },
}

#[cfg(test)]
mod tests {
    use crate::{IntervalStrictFiniteIncInc, Length};
    use errgonomic::handle_bool;
    use thiserror::Error;

    #[test]
    fn length_returns_none_when_strict_interval_length_overflows_output_type() -> Result<(), LengthReturnsNoneWhenStrictIntervalLengthOverflowsOutputTypeError> {
        use LengthReturnsNoneWhenStrictIntervalLengthOverflowsOutputTypeError::*;
        let interval = IntervalStrictFiniteIncInc::new_ordered(i8::MIN, i8::MAX);
        let length = interval.length();
        handle_bool!(length.is_some(), LengthMustOverflowInvalid, length);

        let direct_difference = i8::MAX.checked_sub(i8::MIN);
        handle_bool!(direct_difference.is_some(), DirectDifferenceMustOverflowInvalid, direct_difference);
        Ok(())
    }

    #[derive(Error, Clone, Copy, Debug)]
    pub enum LengthReturnsNoneWhenStrictIntervalLengthOverflowsOutputTypeError {
        #[error("strict interval length must overflow the output type")]
        LengthMustOverflowInvalid { length: Option<i8> },
        #[error("direct i8 subtraction must overflow the output type")]
        DirectDifferenceMustOverflowInvalid { direct_difference: Option<i8> },
    }
}
