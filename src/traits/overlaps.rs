pub trait Overlaps<T> {
    fn overlaps(&self, other: &T) -> bool;
}
