// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_az

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N % 4 == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }
}