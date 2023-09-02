// https://atcoder.jp/contests/joi2015yo/tasks/joi2015yo_a

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
        P: isize,
    }
    println!("{}", min(A * P, B + max(0, P - C) * D));
}