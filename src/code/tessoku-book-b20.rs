// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cs

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let N = S.len();
    let M = T.len();
    let S = S.chars().collect::<Vec<char>>();
    let T = T.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![1<<60; M+1]; N+1];
    for i in 0..=N {
        DP[i][0] = i;
    }
    for j in 0..=M {
        DP[0][j] = j;
    }
    for i in 1..=N {
        for j in 1..=M {
            DP[i][j] = min(DP[i-1][j], DP[i][j-1]) + 1;
            if S[i-1] == T[j-1] {
                DP[i][j] = min(DP[i][j], DP[i-1][j-1]);
            }
            else {
                DP[i][j] = min(DP[i][j], DP[i-1][j-1] + 1);
            }
        }
    }
    println!("{:?}", DP[N][M]);
}