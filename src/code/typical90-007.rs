// https://atcoder.jp/contests/typical90/tasks/typical90_g

use proconio::input;
use proconio::fastout;
use std::cmp::min;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
    }
    A.sort();
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            B: isize,
        }
        let idx = bisect_left(&A, &B);
        let mut ans = 1<<50;
        if 0 < idx {
            ans = min(ans, (A[idx-1] - B).abs());
        }
        if idx < N {
            ans = min(ans, (A[idx] - B).abs());
        }
        println!("{}", ans);
    }
}