// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A < B {
        println!("-1");
    }
    else if A > B {
        println!("1");
    }
    else {
        println!("0");
    }
}