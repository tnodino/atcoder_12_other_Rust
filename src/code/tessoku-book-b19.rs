// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cr

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
    }
    let M = N * 1000;
    let mut DP = vec![vec![1<<60; M+1]; N+1];
    DP[0][0] = 0;
    for i in 0..N {
        input! {
            w: usize,
            v: usize,
        }
        for j in 0..=M {
            DP[i+1][j] = min(DP[i+1][j], DP[i][j]);
            if j + v <= M {
                DP[i+1][j+v] = min(DP[i+1][j+v], DP[i][j] + w);
            }
        }
    }
    for j in (0..=M).rev() {
        if DP[N][j] <= W {
            println!("{}", j);
            return;
        }
    }
}