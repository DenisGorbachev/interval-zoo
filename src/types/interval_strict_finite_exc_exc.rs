use crate::{Contains, Overlaps};
use core::fmt::Debug;
use derive_getters::Getters;
use derive_more::Into;
use thiserror::Error;

#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalStrictFiniteExcExc<T> {
    a: T,
    b: T,
}

impl<T> IntervalStrictFiniteExcExc<T>
where
    T: Ord,
{
    pub fn new_normalized((a, b): (T, T)) -> Self {
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

impl<T> TryFrom<(T, T)> for IntervalStrictFiniteExcExc<T>
where
    T: Ord + Debug,
{
    type Error = TryFromTupleForIntervalStrictFiniteExcExcError<T>;

    fn try_from((a, b): (T, T)) -> Result<Self, Self::Error> {
        use TryFromTupleForIntervalStrictFiniteExcExcError::*;
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

impl<T> Contains<T> for IntervalStrictFiniteExcExc<T>
where
    T: Ord,
{
    fn contains(&self, value: &T) -> bool {
        self.a < *value && *value < self.b
    }
}

impl<T> Overlaps<Self> for IntervalStrictFiniteExcExc<T>
where
    T: Ord,
{
    fn overlaps(&self, other: &Self) -> bool {
        self.a < other.b && other.a < self.b
    }
}

#[derive(Error, Clone, Copy, Debug)]
pub enum TryFromTupleForIntervalStrictFiniteExcExcError<T> {
    #[error("interval lower bound must be less than or equal to upper bound")]
    OrderCheckFailed { a: T, b: T },
}
