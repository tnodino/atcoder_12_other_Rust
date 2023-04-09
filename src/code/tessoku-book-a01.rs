// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N * N);
}