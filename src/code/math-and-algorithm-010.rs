// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_j

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 1;
    for i in 1..=N {
        ans *= i;
    }
    println!("{}", ans);
}