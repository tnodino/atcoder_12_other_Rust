// https://atcoder.jp/contests/typical90/tasks/typical90_h

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let M = 7;
    let S = S.chars().collect::<Vec<char>>();
    let T = "atcoder".chars().collect::<Vec<char>>();
    let mut DP = vec![vec![0; M+1]; N+1];
    DP[0][0] = 1;
    for i in 0..N {
        for j in 0..=M {
            DP[i+1][j] += DP[i][j];
            if j < M && S[i] == T[j] {
                DP[i+1][j+1] += DP[i][j];
            }
        }
        for j in 0..=M {
            DP[i+1][j] %= MOD;
        }
    }
    println!("{}", DP[N][M]);
}