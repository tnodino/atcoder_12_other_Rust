// https://atcoder.jp/contests/dp/tasks/dp_s

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: String,
        D: usize,
    }
    let N = K.len();
    let K = K.chars().collect::<Vec<char>>();
    let mut DP1 = vec![vec![0; D]; N+1];
    let mut DP2 = vec![vec![0; D]; N+1];
    DP2[0][0] = 1;
    for i in 0..N {
        for j in 0..D {
            for k in 0..10 {
                DP1[i+1][(j+k)%D] += DP1[i][j];
            }
        }
        let x = (K[i] as usize) - ('0' as usize);
        for j in 0..D {
            for k in 0..x {
                DP1[i+1][(j+k)%D] += DP2[i][j];
            }
        }
        for j in 0..D {
            DP2[i+1][(j+x)%D] += DP2[i][j];
        }
        for j in 0..D {
            DP1[i+1][j] %= MOD;
        }
    }
    println!("{}", (DP1[N][0] + DP2[N][0] + MOD - 1) % MOD);
}