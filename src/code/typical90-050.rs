// https://atcoder.jp/contests/typical90/tasks/typical90_ax

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
    }
    let mut DP = vec![0; N+1];
    DP[0] = 1;
    for i in 1..=N {
        DP[i] += DP[i-1];
        if L <= i {
            DP[i] += DP[i-L];
        }
        DP[i] %= MOD;
    }
    println!("{}", DP[N]);
}