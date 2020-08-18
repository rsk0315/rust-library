use std::ops::{Add, Mul};

pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn set_zero(&mut self) {
        *self = Zero::zero();
    }
    fn is_zero(&self) -> bool;
}

pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
    fn set_one(&mut self) {
        *self = One::one();
    }
    fn is_one(&self) -> bool;
}

macro_rules! impl_id {
    ( $($ty:ty,)* ) => { $(
        impl Zero for $ty {
            fn zero() -> Self {
                return 0 as $ty;
            }
            fn is_zero(&self) -> bool {
                return *self == 0 as $ty;
            }
        }
        impl One for $ty {
            fn one() -> Self {
                return 1 as $ty;
            }
            fn is_one(&self) -> bool {
                return *self == 1 as $ty;
            }
        }
    )* }
}

impl_id! {
    i8, i16, i32, i64, i128,
    u8, u16, u32, u64, u128,
}
