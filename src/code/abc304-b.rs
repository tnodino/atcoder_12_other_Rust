// https://atcoder.jp/contests/abc304/tasks/abc304_b

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let M = N.to_string().len();
    if M >= 4 {
        N = N - (N % pow(10, M - 3));
    }
    println!("{}", N);
}