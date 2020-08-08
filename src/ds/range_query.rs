pub trait Fold<R> {
    type Output;
    fn fold(&self, irange: R) -> Self::Output;
}
