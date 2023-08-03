// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dp

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; M],
    }
    let mut cnt = vec![0; N];
    for i in 0..M {
        cnt[A[i]-1] += 1;
    }
    for i in 0..N {
        println!("{}", M - cnt[i]);
    }
}