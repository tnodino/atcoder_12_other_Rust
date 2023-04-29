// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_i

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![false; S+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        for j in 0..=S {
            DP[i+1][j] |= DP[i][j];
            if j + A[i] <= S {
                DP[i+1][j+A[i]] |= DP[i][j];
            }
        }
    }
    if DP[N][S] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}