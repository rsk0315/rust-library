use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Neg};

use crate::algebra::Zero;
use crate::algebra::{
    AbelianGroup, AbelianMonoid, Group, Magma, Monoid, Semigroup,
};

pub struct Additive<T> {
    p_: PhantomData<T>,
}

impl<T: Copy + Add<Output = T> + AddAssign> Magma for Additive<T> {
    type Set = T;
    fn op(lhs: T, rhs: T) -> T {
        lhs + rhs
    }
    fn op_assign(lhs: &mut T, rhs: T) {
        *lhs += rhs;
    }
}
impl<T: Copy + Add<Output = T> + AddAssign> Semigroup for Additive<T> {}
impl<T: Copy + Add<Output = T> + AddAssign + Zero> Monoid for Additive<T> {
    fn id() -> T {
        T::zero()
    }
}
impl<T> Group for Additive<T>
where
    T: Copy + Add<Output = T> + AddAssign + Zero + Neg<Output = T>,
{
    fn inv(x: T) -> T {
        -x
    }
}
impl<T> AbelianMonoid for Additive<T> where
    T: Copy + Add<Output = T> + AddAssign + Zero
{
}
impl<T> AbelianGroup for Additive<T> where
    T: Copy + Add<Output = T> + AddAssign + Zero + Neg<Output = T>
{
}
