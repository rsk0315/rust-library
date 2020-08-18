// sample code for testing bundles

use my::algebra::{Additive, Multiplicative};
use my::ds::VecSegtree;
use my::ds::{Fold, Set};

// use my::hack::neko;

fn main() {
    {
        let st: VecSegtree<Additive<i32>> = vec![1, 2, 3, 4, 5].into();
        assert_eq!(14, st.fold(1..));
        assert_eq!(3, st.fold(..2));
        assert_eq!(10, st.fold(0..4));
        assert_eq!(9, st.fold(1..=3));
        assert_eq!(6, st.fold(..=2));
        assert_eq!(15, st.fold(..));
    }

    {
        let st: VecSegtree<Multiplicative<i32>> = vec![9, 2, 4, 7, 3].into();
        assert_eq!(168, st.fold(1..));
        assert_eq!(18, st.fold(..2));
        assert_eq!(504, st.fold(0..4));
        assert_eq!(56, st.fold(1..=3));
        assert_eq!(72, st.fold(..=2));
        assert_eq!(1512, st.fold(..));
    }
}
