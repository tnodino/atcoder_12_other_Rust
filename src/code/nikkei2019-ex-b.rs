// https://atcoder.jp/contests/nikkei2019-ex/tasks/nikkei2019ex_b

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", sqrt(N as f64) as usize);
}