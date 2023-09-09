// https://atcoder.jp/contests/dp/tasks/dp_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        h: [isize; N],
    }
    let mut DP = vec![INF; N];
    DP[0] = 0;
    for i in 1..N {
        if i >= 2 {
            DP[i] = min(DP[i-1] + (h[i] - h[i-1]).abs(), DP[i-2] + (h[i] - h[i-2]).abs());
        }
        else {
            DP[i] = DP[i-1] + (h[i] - h[i-1]).abs();
        }
    }
    println!("{}", DP[N-1]);
}