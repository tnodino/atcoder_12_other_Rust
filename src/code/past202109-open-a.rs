// https://atcoder.jp/contests/past202109-open/tasks/past202109_a

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    println!("{}", min(A + B - C, D));
}