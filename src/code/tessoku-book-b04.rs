// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cc

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    println!("{}", usize::from_str_radix(&N, 2).unwrap());
}