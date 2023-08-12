// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ba

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for i in 1..=60 {
        if N == pow(2, i) - 1 {
            println!("Second");
            return;
        }
    }
    println!("First");
}