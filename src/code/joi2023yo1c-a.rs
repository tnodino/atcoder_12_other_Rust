// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A * 10 + B);
}