// https://atcoder.jp/contests/dp/tasks/dp_e

use proconio::input;
use proconio::fastout;
use std::usize::MAX;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
    }
    let M = 1_000 * N;
    let mut DP = vec![vec![MAX; M+1]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        input! {
            w: usize,
            v: usize,
        }
        for j in 0..=M {
            if DP[i][j] == MAX {
                continue;
            }
            DP[i+1][j] = min(DP[i+1][j], DP[i][j]);
            if j + v <= M {
                DP[i+1][j+v] = min(DP[i+1][j+v], DP[i][j] + w);
            }
        }
    }
    for i in (0..=M).rev() {
        if DP[N][i] <= W {
            println!("{}", i);
            return;
        }
    }
}