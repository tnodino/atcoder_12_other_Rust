// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bz

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A + B);
}