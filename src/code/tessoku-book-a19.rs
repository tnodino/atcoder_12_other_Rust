// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_s

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
    }
    let mut DP = vec![vec![0; W+1]; N+1];
    for i in 0..N {
        input! {
            w: usize,
            v: usize,
        }
        for j in 0..=W {
            DP[i+1][j] = max(DP[i+1][j], DP[i][j]);
            if j + w <= W {
                DP[i+1][j+w] = max(DP[i+1][j+w], DP[i][j] + v);
            }
        }
    }
    println!("{}", DP[N][W]);
}