use std::marker::PhantomData;
use std::ops::{Div, Mul, MulAssign};

use crate::algebra::One;
use crate::algebra::{
    AbelianGroup, AbelianMonoid, Group, Magma, Monoid, Semigroup,
};

pub struct Multiplicative<T> {
    p_: PhantomData<T>,
}

impl<T: Copy + Mul<Output = T> + MulAssign> Magma for Multiplicative<T> {
    type Set = T;
    fn op(lhs: T, rhs: T) -> T {
        lhs * rhs
    }
    fn op_assign(lhs: &mut T, rhs: T) {
        *lhs *= rhs;
    }
}
impl<T: Copy + Mul<Output = T> + MulAssign> Semigroup for Multiplicative<T> {}
impl<T: Copy + Mul<Output = T> + MulAssign + One> Monoid for Multiplicative<T> {
    fn id() -> T {
        T::one()
    }
}
impl<T> Group for Multiplicative<T>
where
    T: Copy + Mul<Output = T> + MulAssign + One + Div<Output = T>,
{
    fn inv(x: T) -> T {
        T::one() / x
    }
}
impl<T> AbelianMonoid for Multiplicative<T> where
    T: Copy + Mul<Output = T> + MulAssign + One
{
}
impl<T> AbelianGroup for Multiplicative<T> where
    T: Copy + Mul<Output = T> + MulAssign + One + Div<Output = T>
{
}
