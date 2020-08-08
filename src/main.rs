// sample code for testing bundles

use my::algebra::Additive;
use my::ds::vec_segtree::*;

fn main() {
    let st: VecSegtree<Additive<i32>> = vec![1, 2, 3, 4, 5].into();
    println!("{}", st.fold(1..));
    println!("{}", st.fold(..2));
    println!("{}", st.fold(0..4));
    println!("{}", st.fold(..));
}
