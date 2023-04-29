// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_f

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", 2 * N + 3);
}