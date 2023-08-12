// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    if A + B * 7 <= 30 {
        println!("1");
    }
    else {
        println!("0");
    }
}