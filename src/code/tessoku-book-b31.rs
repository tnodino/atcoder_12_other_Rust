// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dd

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N / 3 + N / 5 + N / 7 - N / 15 - N / 21 - N / 35 + N / 105);
}