// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ap

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP = vec![0; N+1];
    DP[1] = 1;
    DP[2] = 1;
    for i in 3..=N {
        DP[i] = DP[i-1] + DP[i-2];
        DP[i] %= MOD;
    }
    println!("{}", DP[N]);
}