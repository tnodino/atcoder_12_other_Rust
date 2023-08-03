// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_af

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
    }
    let mut DP = vec![false; N+1];
    for i in 1..=N {
        if A <= i && !DP[i-A] {
            DP[i] = true;
        }
        if B <= i && !DP[i-B] {
            DP[i] = true;
        }
    }
    if DP[N] {
        println!("First");
    }
    else {
        println!("Second");
    }
}