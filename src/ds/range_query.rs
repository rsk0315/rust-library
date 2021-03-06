//! Range queries.
//! - [`Fold`]
//!   - `.fold(il..ir) -> T`
//! - [`Set`]
//!   - `.set(i, x)`
//! - [`Act`]
//!   - `.act(i, x)`
//!   - `.act(il..ir, x)`
//! - [`Rank`]
//!   - `.rank(il..ir, x) -> usize`
//! - [`Select`]
//!   - `.select(il..ir, x) -> Option<usize>`
//! - [`Quantile`]
//!   - `.quantile(il..ir, x) -> T`
//! - [`RangeFreq`]
//!   - `.rangefreq(il..ir, xl..xr) -> usize`
//! - [`OrdedCount`]
//!   - `.orded_count(il..ir, x) -> Orded<usize>`
//! - [`OrdedSum`]
//!   - `.orded_sum(il..ir, x) -> Orded<T>`
//!
//! Range traits.
//! - Start{Bounded, Unbounded}
//! - End{Bounded, Unbounded}
//!
//! Inclusive and Exclusive for Bounded ones.
//!
//! Orded struct.
//! `{ lt: T, le: T, eq: T, gt: T, ge: T }`

use std::ops::Bound::*;
use std::ops::RangeBounds;
use std::ops::{
    Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

pub trait StartBounded<T>: RangeBounds<T> {}
pub trait StartInclusive<T>: StartBounded<T> {}
pub trait StartUnbounded<T>: RangeBounds<T> {}
pub trait EndBounded<T>: RangeBounds<T> {}
pub trait EndExclusive<T>: EndBounded<T> {}
pub trait EndInclusive<T>: EndBounded<T> {}
pub trait EndUnbounded<T>: RangeBounds<T> {}

impl<T> StartBounded<T> for Range<T> {}
impl<T> EndBounded<T> for Range<T> {}
impl<T> StartBounded<T> for RangeFrom<T> {}
impl<T> EndUnbounded<T> for RangeFrom<T> {}
impl<T> StartUnbounded<T> for RangeFull {}
impl<T> EndUnbounded<T> for RangeFull {}
impl<T> StartBounded<T> for RangeInclusive<T> {}
impl<T> EndBounded<T> for RangeInclusive<T> {}
impl<T> StartUnbounded<T> for RangeTo<T> {}
impl<T> EndBounded<T> for RangeTo<T> {}
impl<T> StartUnbounded<T> for RangeToInclusive<T> {}
impl<T> EndBounded<T> for RangeToInclusive<T> {}

impl<T> StartInclusive<T> for Range<T> {}
impl<T> EndExclusive<T> for Range<T> {}
impl<T> StartInclusive<T> for RangeFrom<T> {}
impl<T> StartInclusive<T> for RangeInclusive<T> {}
impl<T> EndInclusive<T> for RangeInclusive<T> {}
impl<T> EndExclusive<T> for RangeTo<T> {}
impl<T> EndInclusive<T> for RangeToInclusive<T> {}

pub trait BufRange: RangeBounds<usize> {
    fn bounds_within(&self, len: usize) -> (usize, usize) {
        let s_in = match self.start_bound() {
            Included(&i) => i,
            Excluded(&i) => i + 1,
            Unbounded => 0,
        };
        let e_ex = match self.end_bound() {
            Included(&i) => i + 1,
            Excluded(&i) => i,
            Unbounded => len,
        };
        (s_in, e_ex)
    }
}

impl BufRange for Range<usize> {}
impl BufRange for RangeFrom<usize> {}
impl BufRange for RangeFull {}
impl BufRange for RangeInclusive<usize> {}
impl BufRange for RangeTo<usize> {}
impl BufRange for RangeToInclusive<usize> {}

pub trait Fold<R: BufRange> {
    type Output;
    fn fold(&self, irange: R) -> Self::Output;
}

pub trait Set<I> {
    fn set(&self, index: I);
}
