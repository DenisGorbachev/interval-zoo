use derive_getters::Getters;
use derive_more::Into;

#[derive(Getters, Into, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IntervalStrictFiniteExcExc<T> {
    a: T,
    b: T,
}
