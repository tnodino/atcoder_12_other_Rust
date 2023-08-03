// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_de

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; K],
    }
    let mut DP = vec![false; N+1];
    for i in 1..=N {
        for k in 0..K {
            if a[k] <= i && !DP[i-a[k]] {
                DP[i] = true;
            }
        }
    }
    if DP[N] {
        println!("First");
    }
    else {
        println!("Second");
    }
}