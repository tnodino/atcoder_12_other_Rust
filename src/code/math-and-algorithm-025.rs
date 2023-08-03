// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_y

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [f64; N],
        B: [f64; N],
    }
    let mut ans = 0.;
    for i in 0..N {
        ans += A[i] * (1. / 3.);
        ans += B[i] * (2. / 3.);
    }
    println!("{}", ans);
}