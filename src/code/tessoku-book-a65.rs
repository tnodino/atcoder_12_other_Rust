// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bm

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
    }
    let mut DP = vec![0; N];
    for i in (1..N).rev() {
        DP[A[i-1]-1] += DP[i] + 1;
    }
    println!("{}", DP.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}