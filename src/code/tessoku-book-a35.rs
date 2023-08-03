// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ai

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![0; N]; N];
    for j in 0..N {
        DP[N-1][j] = A[j];
    }
    for i in (0..N-1).rev() {
        for j in 0..=i {
            if i % 2 == 0 {
                DP[i][j] = max(DP[i+1][j], DP[i+1][j+1]);
            }
            else {
                DP[i][j] = min(DP[i+1][j], DP[i+1][j+1]);
            }
        }
    }
    println!("{}", DP[0][0]);
}