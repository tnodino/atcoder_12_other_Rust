// https://atcoder.jp/contests/typical90/tasks/typical90_s

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: isize = 1<<50;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N*2],
    }
    let M = N * 2;
    let mut DP = vec![vec![INF; M]; M];
    for i in 0..M-1 {
        DP[i][i+1] = (A[i] - A[i+1]).abs();
    }
    for i in (1..M).step_by(2) {
        for j in 0..M-i {
            let l = j;
            let r = j + i;
            for k in l..r {
                DP[l][r] = min(DP[l][r], DP[l][k] + DP[k+1][r]);
            }
            DP[l][r] = min(DP[l][r], DP[l+1][r-1] + (A[l] - A[r]).abs());
        }
    }
    println!("{}", DP[0][M-1]);
}