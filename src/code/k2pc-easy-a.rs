// https://atcoder.jp/contests/k2pc-easy/tasks/k2pc001_e1

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        N: isize,
    }
    println!("{} {} {}", max(0, N - a), max(0, N * 2 - b), max(0, N * 3 - c));
}