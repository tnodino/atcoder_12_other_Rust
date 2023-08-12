// https://atcoder.jp/contests/joisc2007/tasks/joisc2007_buildi

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
        n: usize,
    }
    let mut DP = vec![INF; n+1];
    for _ in 0..n {
        input! {
            a: usize,
        }
        let idx = bisect_left(&DP, &a);
        DP[idx] = a;
    }
    for i in 1..=n {
        if DP[i] == INF {
            println!("{}", i);
            break;
        }
    }
}