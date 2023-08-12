// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_bc

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let s = A.iter().sum::<usize>();
    if s > K || s % 2 != K % 2 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}