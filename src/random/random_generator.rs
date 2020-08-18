pub trait RandomGenerator {
    type Output;
    const MIN: Self::Output;
    const MAX: Self::Output;
    fn next(&mut self) -> Self::Output;
}
