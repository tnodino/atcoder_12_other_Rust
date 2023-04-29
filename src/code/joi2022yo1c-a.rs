// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", B - A);
}