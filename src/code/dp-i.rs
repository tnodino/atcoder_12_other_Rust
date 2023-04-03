// https://atcoder.jp/contests/dp/tasks/dp_i

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [f64; N],
    }
    let mut DP = vec![vec![0.; N+1]; N+1];
    DP[0][0] = 1.;
    for i in 0..N {
        for j in 0..N {
            DP[i+1][j+1] += DP[i][j] * p[i];
            DP[i+1][j] += DP[i][j] * (1. - p[i]);
        }
    }
    let M = (N + 2) / 2;
    let mut ans = 1.;
    for i in 0..M {
        ans -= DP[N][i];
    }
    println!("{}", ans);
}