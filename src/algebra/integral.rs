use std::cmp::*;
use std::fmt::*;
use std::ops::*;

pub trait Integral:
    Clone
    + Debug
    + Display
    + Sized
    + Add<Output = Self>
    + AddAssign
    + Mul<Output = Self>
    + MulAssign
    + Sub<Output = Self>
    + SubAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Eq
    + Ord
{
}
pub trait Binary:
    Integral
    + BitAnd
    + BitAndAssign
    + BitOr
    + BitOrAssign
    + BitXor
    + BitXorAssign
    + Shl
    + Shl
{
}
macro_rules! impl_empty_trait {
    ( $( $tr:ident: $( $ty:tt ),* ;)* ) => {
        $( $( impl $tr for $ty {} )* )*
    }
}

impl_empty_trait! {
    Integral: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
    Binary: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize;
}
