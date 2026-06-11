use crate::{Contains, Overlaps};
use core::fmt::Debug;
use derive_getters::Getters;
use derive_more::Into;
use thiserror::Error;

pub const INTERVAL_BOUND_EXCLUDED: bool = false;
pub const INTERVAL_BOUND_INCLUDED: bool = true;

#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalStrictFinite<T, const A_INC: bool, const B_INC: bool> {
    a: T,
    b: T,
}

pub type IntervalStrictFiniteExcExc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_EXCLUDED>;
pub type IntervalStrictFiniteExcInc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_EXCLUDED, INTERVAL_BOUND_INCLUDED>;
pub type IntervalStrictFiniteIncExc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_EXCLUDED>;
pub type IntervalStrictFiniteIncInc<T> = IntervalStrictFinite<T, INTERVAL_BOUND_INCLUDED, INTERVAL_BOUND_INCLUDED>;

impl<T, const A_INC: bool, const B_INC: bool> IntervalStrictFinite<T, A_INC, B_INC>
where
    T: Ord,
{
    pub fn new_ordered((a, b): (T, T)) -> Self {
        use core::cmp::Ordering::*;
        match a.cmp(&b) {
            Greater => Self {
                a: b,
                b: a,
            },
            Equal | Less => Self {
                a,
                b,
            },
        }
    }
}

impl<T, const A_INC: bool, const B_INC: bool> TryFrom<(T, T)> for IntervalStrictFinite<T, A_INC, B_INC>
where
    T: Ord + Debug,
{
    type Error = TryFromTupleForIntervalStrictFiniteError<T>;

    fn try_from((a, b): (T, T)) -> Result<Self, Self::Error> {
        use TryFromTupleForIntervalStrictFiniteError::*;
        if a <= b {
            Ok(Self {
                a,
                b,
            })
        } else {
            Err(OrderCheckFailed {
                a,
                b,
            })
        }
    }
}

macro_rules! impl_contains {
    ($a_inc:expr, $b_inc:expr, $lower_op:tt, $upper_op:tt) => {
        impl<T> Contains<T> for IntervalStrictFinite<T, $a_inc, $b_inc>
        where
            T: Ord,
        {
            fn contains(&self, value: &T) -> bool {
                self.a $lower_op *value && *value $upper_op self.b
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
                self.a $left_op other.b && other.a $right_op self.b
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
    OrderCheckFailed { a: T, b: T },
}
