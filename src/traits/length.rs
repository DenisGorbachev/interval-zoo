pub trait Length {
    type Output;

    fn length(&self) -> Self::Output;
}
