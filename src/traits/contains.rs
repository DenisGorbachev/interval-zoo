pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;
}
