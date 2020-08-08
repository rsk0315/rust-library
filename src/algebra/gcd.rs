use std::marker::PhantomData;
use std::mem;
use std::ops::RemAssign;

use num_traits::Zero;

use crate::algebra::{AbelianMonoid, Magma, Monoid, Semigroup};

fn gcd<T: Copy + RemAssign + Zero>(mut n: T, mut m: T) -> T {
    while !m.is_zero() {
        n %= m;
        mem::swap(&mut n, &mut m);
    }
    n
}

pub struct Gcd<T> {
    p_: PhantomData<T>,
}

impl<T: Copy + RemAssign + Zero> Magma for Gcd<T> {
    type Set = T;
    fn op(lhs: T, rhs: T) -> T {
        gcd(lhs, rhs)
    }
    fn op_assign(lhs: &mut T, rhs: T) {
        *lhs = Self::op(*lhs, rhs);
    }
}
impl<T: Copy + RemAssign + Zero> Semigroup for Gcd<T> {}
impl<T: Copy + RemAssign + Zero> Monoid for Gcd<T> {
    fn id() -> T {
        T::zero()
    }
}
impl<T: Copy + RemAssign + Zero> AbelianMonoid for Gcd<T> {}
