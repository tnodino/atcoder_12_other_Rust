// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", X * 24);
}