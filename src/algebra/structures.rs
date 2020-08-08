pub trait Magma {
    type Set: Copy;
    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set;
    fn op_assign(lhs: &mut Self::Set, rhs: Self::Set);
}
pub trait Semigroup: Magma {
    // Operation should be associative.
}
pub trait Monoid: Semigroup {
    fn id() -> Self::Set;
}
pub trait Group: Monoid {
    fn inv(x: Self::Set) -> Self::Set;
}
pub trait AbelianMonoid: Monoid {
    // Operation should be commutative.
}
pub trait AbelianGroup: Group {
    // Operation should be commutative.
}
