// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ax

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        X: isize,
        Y: isize,
    }
    let X = X.abs();
    let Y = Y.abs();
    if X + Y <= N && (X + Y) % 2 == N % 2 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}