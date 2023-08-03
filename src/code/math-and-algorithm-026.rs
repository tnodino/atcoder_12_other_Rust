// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_z

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut exp = 0.;
    for i in 1..=N {
        exp += 1. / (i as f64);
    }
    exp *= N as f64;
    println!("{}", exp);
}