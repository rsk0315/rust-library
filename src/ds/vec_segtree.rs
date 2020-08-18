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

#[cfg(test)]
mod tests {
    use crate::ds::Fold;
    use crate::ds::VecSegtree;

    #[test]
    fn test_add() {
        use crate::algebra::Additive;
        let st: VecSegtree<Additive<i32>> = vec![1, 2, 3, 4, 5].into();
        assert_eq!(14, st.fold(1..));
        assert_eq!(3, st.fold(..2));
        assert_eq!(10, st.fold(0..4));
        assert_eq!(9, st.fold(1..=3));
        assert_eq!(6, st.fold(..=2));
        assert_eq!(15, st.fold(..));
    }

    #[test]
    fn test_mul() {
        use crate::algebra::Multiplicative;
        let st: VecSegtree<Multiplicative<i32>> = vec![9, 2, 4, 7, 3].into();
        assert_eq!(168, st.fold(1..));
        assert_eq!(18, st.fold(..2));
        assert_eq!(504, st.fold(0..4));
        assert_eq!(56, st.fold(1..=3));
        assert_eq!(72, st.fold(..=2));
        assert_eq!(1512, st.fold(..));
    }

    #[test]
    fn test_random() {
        use crate::algebra::Additive;
        use crate::random::uniform_int_dist;
        use crate::random::xorshift::*;

        let mut xs = Xorshift128::new([12, 34, 56, 78]);
        let n = 1000;
        let base: Vec<u64> = (0..n)
            .map(|_| uniform_int_dist(0..=1000, &mut xs) as u64)
            .collect();
        let st: VecSegtree<Additive<u64>> = base.clone().into();

        println!("{:?}", base);

        for il in 0..n {
            let mut sum: u64 = 0;
            assert_eq!(0, st.fold(il..il));
            for ir in il..n {
                sum += base[ir];
                assert_eq!(sum, st.fold(il..ir + 1));
            }
            assert_eq!(sum, st.fold(il..));
        }
        assert_eq!(0, st.fold(n..n));
        assert_eq!(0, st.fold(n..));

        assert_eq!(0, st.fold(..0));
        let mut sum: u64 = 0;
        for ir in 0..n {
            sum += base[ir];
            assert_eq!(sum, st.fold(..ir + 1));
        }
        assert_eq!(sum, st.fold(..n));
        assert_eq!(sum, st.fold(..));
    }
}
