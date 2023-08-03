// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ac

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![0; 2]; N+1];
    for i in 0..N {
        DP[i+1][0] = max(DP[i][0], DP[i][1]);
        DP[i+1][1] = DP[i][0] + A[i];
    }
    println!("{}", max(DP[N][0], DP[N][1]));
}