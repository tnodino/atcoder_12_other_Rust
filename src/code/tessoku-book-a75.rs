// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bw

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1_440;
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            T: usize,
            D: usize,
        }
        vec.push((D, T));
    }
    vec.sort();
    let mut DP = vec![vec![0; M+1]; N+1];
    for i in 0..N {
        for j in 0..=M {
            DP[i+1][j] = max(DP[i+1][j], DP[i][j]);
            if j + vec[i].1 <= vec[i].0 {
                DP[i+1][j+vec[i].1] = max(DP[i+1][j+vec[i].1], DP[i][j] + 1);
            }
        }
    }
    let mut ans = 0;
    for j in 0..=M {
        ans = max(ans, DP[N][j]);
    }
    println!("{}", ans);
}