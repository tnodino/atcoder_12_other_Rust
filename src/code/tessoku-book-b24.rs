// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cw

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
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
        }
        vec.push((X, Y));
    }
    vec.sort_by(|a, b| if a.0 == b.0 {
        b.1.cmp(&a.1)
    }
    else {
        a.0.cmp(&b.0)
    });
    let mut DP = vec![INF; N];
    for i in 0..N {
        let idx = bisect_left(&DP, &vec[i].1);
        DP[idx] = vec[i].1;
    }
    for i in (0..N).rev() {
        if DP[i] != INF {
            println!("{}", i+1);
            return;
        }
    }
}