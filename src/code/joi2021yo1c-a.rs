// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_a

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    println!("{}", max(A + B, A - B));
    println!("{}", min(A + B, A - B));
}