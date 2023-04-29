// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
    }
    println!("{}", A1 + A2 + A3);
}