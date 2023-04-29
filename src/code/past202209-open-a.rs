// https://atcoder.jp/contests/past202209-open/tasks/past202209_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
    }
    println!("{}", max(Y, X + Z));
}