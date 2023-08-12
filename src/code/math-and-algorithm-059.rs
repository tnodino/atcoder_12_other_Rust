// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ay

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let ary = [6, 2, 4, 8];
    println!("{}", ary[N % 4]);
}