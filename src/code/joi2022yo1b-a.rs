// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", X * X * X);
}