// https://atcoder.jp/contests/joig2021-open/tasks/joig2021_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let ma = max(A, max(B, C));
    println!("{}", ma * 3 - A - B - C);
}