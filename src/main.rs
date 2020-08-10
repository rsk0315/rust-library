// sample code for testing bundles

use my::algebra::Additive;
use my::ds::Fold;
use my::ds::VecSegtree;

fn main() {
    let st: VecSegtree<Additive<i32>> = vec![1, 2, 3, 4, 5].into();
    assert_eq!(14, st.fold(1..));
    assert_eq!(3, st.fold(..2));
    assert_eq!(10, st.fold(0..4));
    assert_eq!(9, st.fold(1..=3));
    assert_eq!(6, st.fold(..=2));
    assert_eq!(15, st.fold(..));
}
