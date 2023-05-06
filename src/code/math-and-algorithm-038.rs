// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ai

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }
    let mut B = vec![0; N+1];
    for i in 0..N {
        B[i+1] += A[i] + B[i];
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
        }
        println!("{}", B[R] - B[L-1]);
    }
}