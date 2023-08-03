// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ab

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP: Vec<usize> = vec![0; N+1];
    DP[0] = 1;
    for i in 1..=N {
        DP[i] += DP[i-1];
        if 2 <= i {
            DP[i] += DP[i-2];
        }
    }
    println!("{}", DP[N]);
}