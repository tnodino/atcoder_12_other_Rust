// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", (A - 1 + B) % 12 + 1);
}