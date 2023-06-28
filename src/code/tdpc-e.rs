// https://atcoder.jp/contests/tdpc/tasks/tdpc_number

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        N: String,
    }
    let M = N.len();
    let N = N.chars().collect::<Vec<char>>();
    let mut DP1 = vec![vec![0; D]; M+1];
    let mut DP2 = vec![vec![0; D]; M+1];
    DP2[0][0] = 1;
    for i in 0..M {
        for d in 0..D {
            for k in 0..=9 {
                let x = (d + k) % D;
                DP1[i+1][x] += DP1[i][d];
                DP1[i+1][x] %= MOD;
            }
        }
        let num = (N[i] as usize) - 48;
        for d in 0..D {
            for k in 0..num {
                let x = (d + k) % D;
                DP1[i+1][x] += DP2[i][d];
                DP1[i+1][x] %= MOD;
            }
        }
        for d in 0..D {
            let x = (d + num) % D;
            DP2[i+1][x] += DP2[i][d];
            DP2[i+1][x] %= MOD;
        }
    }
    DP1[M][0] = (DP1[M][0] + MOD - 1) % MOD;
    println!("{}", (DP1[M][0] + DP2[M][0]) % MOD);
}