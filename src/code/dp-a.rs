// https://atcoder.jp/contests/dp/tasks/dp_a

use proconio::input;
use proconio::fastout;
use std::isize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        h: [isize; N],
    }
    let mut DP = vec![MAX; N];
    DP[0] = 0;
    for i in 0..N {
        if (i as isize) - 1 >= 0 {
            DP[i] = min(DP[i], DP[i-1] + (h[i-1] - h[i]).abs());
        }
        if (i as isize) - 2 >= 0 {
            DP[i] = min(DP[i], DP[i-2] + (h[i-2] - h[i]).abs());
        }
    }
    println!("{}", DP[N-1]);
}