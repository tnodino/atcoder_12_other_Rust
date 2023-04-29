// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_g

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        if i % X == 0 || i % Y == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}