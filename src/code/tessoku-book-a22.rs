// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_v

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
        B: [usize; N-1],
    }
    let mut DP = vec![-1<<30; N];
    DP[0] = 0;
    for i in 0..N-1 {
        DP[A[i]-1] = max(DP[A[i]-1], DP[i] + 100);
        DP[B[i]-1] = max(DP[B[i]-1], DP[i] + 150);
    }
    println!("{}", DP[N-1]);
}