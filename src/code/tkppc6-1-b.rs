// https://atcoder.jp/contests/tkppc6-1/tasks/tkppc6_1_b

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.dedup();
    println!("{}", min(N, A.len() + M));
}