// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_w

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        B: [f64; N],
        R: [f64; N],
    }
    let M = N as f64;
    let mut b = 0.;
    let mut r = 0.;
    for i in 0..N {
        b += B[i] / M;
        r += R[i] / M;
    }
    println!("{}", b + r);
}