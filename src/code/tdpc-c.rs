// https://atcoder.jp/contests/tdpc/tasks/tdpc_tournament

use proconio::input;
use proconio::fastout;
use libm::pow;

#[allow(non_snake_case)]
fn f(P: f64, Q: f64) -> f64 {
    return 1. / (1. + pow(10., (Q - P) / 400.))
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        R: [f64; 1<<K],
    }
    let N = 1 << K;
    let mut DP = vec![vec![0.; N]; K+1];
    for i in 0..N {
        DP[0][i] = 1.;
    }
    let mut mask = (1 << K) - 1;
    for k in 0..K {
        mask <<= 1;
        for i in 0..N {
            for j in 0..N {
                if i & mask == j & mask && (1 << k) & i != (1 << k) & j {
                    DP[k+1][i] += DP[k][i] * DP[k][j] * f(R[i], R[j]);
                }
            }
        }
    }
    for i in 0..N {
        println!("{}", DP[K][i]);
    }
}