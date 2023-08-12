// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
    }
    if X + Y <= Z {
        println!("1");
    }
    else {
        println!("0");
    }
}