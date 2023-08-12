// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N / 10 == N % 10 {
        println!("1");
    }
    else {
        println!("0");
    }
}