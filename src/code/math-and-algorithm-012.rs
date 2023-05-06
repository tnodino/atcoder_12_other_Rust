// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_l

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    for i in 2..=M {
        if N % i == 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}