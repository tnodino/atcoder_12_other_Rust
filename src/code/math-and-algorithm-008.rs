// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_h

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=N {
            if i + j <= S {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}