// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ez

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    }
    A.sort_by(|a, b| b.cmp(a));
    println!("{}", A[0] + A[1]);
}