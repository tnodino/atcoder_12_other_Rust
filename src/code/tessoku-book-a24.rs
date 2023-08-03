// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_x

use proconio::input;
use proconio::fastout;

const INF: usize = 1<<60;

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
        A: [usize; N],
    }
    let mut DP = vec![INF; N];
    for i in 0..N {
        let idx = bisect_left(&DP, &A[i]);
        DP[idx] = A[i];
    }
    for i in (0..N).rev() {
        if DP[i] != INF {
            println!("{}", i+1);
            return;
        }
    }
}