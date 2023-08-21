// https://atcoder.jp/contests/typical90/tasks/typical90_t

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    if a < pow(c, b) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}