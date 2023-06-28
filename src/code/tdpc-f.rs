// https://atcoder.jp/contests/tdpc/tasks/tdpc_semiexp

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut DP = vec![[0, 0]; N];
    DP[0][1] = 1;
    let mut over = vec![0; N];
    over[0] = 1;
    for i in 1..N {
        DP[i][0] = (DP[i-1][0] + DP[i-1][1]) % MOD;
        DP[i][1] = (DP[i-1][0] + DP[i-1][1]) % MOD;
        over[i] = DP[i-1][0];
        if i + 1 >= K {
            DP[i][1] = (DP[i][1] + MOD - over[i+1-K]) % MOD;
        }
    }
    println!("{}", DP[N-1][1]);
}