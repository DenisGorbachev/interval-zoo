use crate::{Contains, Length, Overlaps};
use core::fmt::Debug;
use derive_getters::Getters;
use derive_more::Into;
use num_traits::CheckedSub;
use thiserror::Error;

pub const INTERVAL_BOUND_EXCLUDED: bool = false;
pub const INTERVAL_BOUND_INCLUDED: bool = true;

/// A strict finite interval.
///
/// This type intentionally doesn't implement `Ord` or `PartialOrd`, because a single interval has multiple values that can be compared (for example: field values, length value). Users should compare the values directly.
#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalStrictFinite<T, const A_INC: bool, const B_INC: bool> {
    lo: T,
    hi: T,
}

pub type IntervalStrictFiniteExcExc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED>;
pub type IntervalStrictFiniteExcInc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED>;
pub type IntervalStrictFiniteIncExc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED>;
pub type IntervalStrictFiniteIncInc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED>;

impl<T, const A_INC: bool, const B_INC: bool> IntervalStrictFinite<T, A_INC, B_INC>
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

impl<T, const A_INC: bool, const B_INC: bool> TryFrom<(T, T)> for IntervalStrictFinite<T, A_INC, B_INC>
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

impl<T, const A_INC: bool, const B_INC: bool> Length for IntervalStrictFinite<T, A_INC, B_INC>
where
    T: CheckedSub,
{
    type Output = Option<T>;

    fn length(&self) -> Self::Output {
        self.hi.checked_sub(&self.lo)
    }
}

macro_rules! impl_contains {
    ($a_inc:expr, $b_inc:expr, $lower_op:tt, $upper_op:tt) => {
        impl<T> Contains<T> for IntervalStrictFinite<T, $a_inc, $b_inc>
        where
            T: Ord,
        {
            fn contains(&self, value: &T) -> bool {
                self.lo $lower_op *value && *value $upper_op self.hi
            }
        }
    };
}

impl_contains!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_contains!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, <, <=);
impl_contains!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, <=, <);
impl_contains!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, <=, <=);

macro_rules! impl_overlaps {
    ($a_inc:expr, $b_inc:expr, $other_a_inc:expr, $other_b_inc:expr, $left_op:tt, $right_op:tt) => {
        impl<T> Overlaps<IntervalStrictFinite<T, $other_a_inc, $other_b_inc>> for IntervalStrictFinite<T, $a_inc, $b_inc>
        where
            T: Ord,
        {
            fn overlaps(&self, other: &IntervalStrictFinite<T, $other_a_inc, $other_b_inc>) -> bool {
                self.lo $left_op other.hi && other.lo $right_op self.hi
            }
        }
    };
}

impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, <, <=);
impl_overlaps!(INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, <, <=);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, <=, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, <=, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED, <, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED, <=, <);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED, <, <=);
impl_overlaps!(INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED, <=, <=);

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromTupleForIntervalStrictFiniteError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { lo: T, hi: T },
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
