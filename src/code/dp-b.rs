// https://atcoder.jp/contests/dp/tasks/dp_b

use proconio::input;
use proconio::fastout;
use std::isize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        h: [isize; N],
    }
    let mut DP = vec![MAX; N];
    DP[0] = 0;
    for i in 0..N {
        for k in 1..=K {
            if (i as isize) - (k as isize) < 0 {
                break;
            }
            DP[i] = min(DP[i], DP[i-k] + (h[i-k] - h[i]).abs());
        }
    }
    println!("{}", DP[N-1]);
}