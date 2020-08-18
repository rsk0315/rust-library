use std::cmp::*;
use std::ops::RangeInclusive;

use num::NumCast;
use num_traits::Num;

use crate::algebra::Integral;
use crate::random::RandomGenerator;

pub fn uniform_int_dist<T, G>(range: RangeInclusive<T>, gen: &mut G) -> T
where
    T: Num + Ord + NumCast + Copy,
    G: RandomGenerator<Output = T>,
{
    let urngmin = G::MIN;
    let urngmax = G::MAX;
    let urngrange = urngmax - urngmin;
    let urange: T = *range.end() - *range.start();
    let ret = match urngrange.cmp(&urange) {
        std::cmp::Ordering::Greater => {
            let uerange = urange + T::from(1).unwrap();
            let scaling = urngrange / uerange;
            let past = uerange * scaling;
            let mut ret = gen.next() - urngmin;
            while ret >= past {
                ret = gen.next() - urngmin;
            }
            ret / scaling
        }
        std::cmp::Ordering::Less => {
            let mut ret: T;
            loop {
                let uerngrange = urngrange + T::from(1).unwrap();
                let tmp = uerngrange
                    * uniform_int_dist(
                        T::from(0).unwrap()..=urange / uerngrange,
                        gen,
                    );
                ret = tmp + gen.next() - urngmin;
                if ret > urange || ret < tmp {
                    break;
                }
            }
            ret
        }
        std::cmp::Ordering::Equal => gen.next() - urngmin,
    };
    ret + *range.start()
}
