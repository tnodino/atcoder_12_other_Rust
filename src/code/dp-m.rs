// https://atcoder.jp/contests/dp/tasks/dp_m

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; N],
    }
    let mut DP = vec![vec![0; K+1]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        let mut s = vec![0; K+1];
        s[0] = DP[i][0];
        for j in 1..=K {
            s[j] = (s[j-1] + DP[i][j]) % MOD;
        }
        for j in 0..=K {
            if j <= a[i] {
                DP[i+1][j] = s[j];
            }
            else {
                DP[i+1][j] = (s[j] + MOD - s[j-a[i]-1]) % MOD;
            }
        }
    }
    println!("{}", DP[N][K]);
}