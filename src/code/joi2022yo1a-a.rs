// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
    }
    println!("{}", X % 21);
}