// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cz

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", lcm(A, B));
}