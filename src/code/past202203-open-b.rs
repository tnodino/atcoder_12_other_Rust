// https://atcoder.jp/contests/past202203-open/tasks/past202203_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        X: usize,
        Y: usize,
    }
    println!("{}", min(X / A, Y / B));
}