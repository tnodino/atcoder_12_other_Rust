// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_x

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0.;
    for _ in 0..N {
        input! {
            P: f64,
            Q: f64,
        }
        ans += Q / P;
    }
    println!("{}", ans);
}