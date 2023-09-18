// https://atcoder.jp/contests/tdpc/tasks/tdpc_number

use proconio::input;
use proconio::fastout;
use ac_library::ModInt1000000007 as Mint;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (D, N): (usize, String),
    }
    let N = N.chars().map(|x| (x as usize) - ('0' as usize)).collect::<Vec<usize>>();
    let M = N.len();
    let mut DP = vec![vec![vec![Mint::new(0); D]; 2]; M+1];
    DP[0][0][0] = 1.into();
    for i in 0..M {
        for k in 0..D {
            for d in 0..10 {
                let idx = (k + d) % D;
                DP[i+1][1][idx] = DP[i+1][1][idx] + DP[i][1][k];
            }
            for d in 0..N[i] {
                let idx = (k + d) % D;
                DP[i+1][1][idx] = DP[i+1][1][idx] + DP[i][0][k];
            }
            let idx = (k + N[i]) % D;
            DP[i+1][0][idx] = DP[i+1][0][idx] + DP[i][0][k];
        }
    }
    println!("{}", DP[M][0][0] + DP[M][1][0] - 1);
}