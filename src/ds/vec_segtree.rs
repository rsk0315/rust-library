use std::convert::From;
use std::iter::FromIterator;

use crate::algebra::Monoid;
use crate::ds::BufRange;
#[doc(hidden)]
pub use crate::ds::Fold;

pub struct VecSegtree<M: Monoid> {
    buf: Vec<M::Set>,
    len: usize,
}

impl<M: Monoid> VecSegtree<M> {
    pub fn new(len: usize) -> Self {
        VecSegtree {
            buf: vec![M::id(); len + len],
            len: len + len,
        }
    }
}
impl<M: Monoid, R: BufRange> Fold<R> for VecSegtree<M> {
    type Output = M::Set;
    fn fold(&self, irange: R) -> M::Set {
        let (mut l, mut r) = irange.bounds_within(self.len);
        let (mut resl, mut resr) = (M::id(), M::id());
        if r > self.len || l >= r {
            return resl;
        }
        l += self.len;
        r += self.len;
        while l < r {
            if l & 1 == 1 {
                M::op_assign(&mut resl, self.buf[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                resr = M::op(self.buf[r], resr);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(resl, resr)
    }
}
impl<M: Monoid> From<Vec<M::Set>> for VecSegtree<M> {
    fn from(mut base: Vec<M::Set>) -> Self {
        let len = base.len();
        let mut buf = vec![M::id(); len];
        buf.append(&mut base);
        for i in (1..len).rev() {
            buf[i] = M::op(buf[i << 1 | 0], buf[i << 1 | 1]);
        }
        VecSegtree { buf, len }
    }
}
impl<M: Monoid> FromIterator<M::Set> for VecSegtree<M> {
    fn from_iter<I: IntoIterator<Item = M::Set>>(iter: I) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}
