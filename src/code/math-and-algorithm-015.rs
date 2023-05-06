// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_o

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", gcd(A, B));
}