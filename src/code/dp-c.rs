// https://atcoder.jp/contests/dp/tasks/dp_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut DP = vec![[0; 3]; N+1];
    for i in 0..N {
        input! {
            a: usize,
            b: usize,
            c: usize,
        }
        DP[i+1][0] = max(DP[i][1], DP[i][2]) + a;
        DP[i+1][1] = max(DP[i][0], DP[i][2]) + b;
        DP[i+1][2] = max(DP[i][0], DP[i][1]) + c;
    }
    println!("{}", DP[N].iter().max().unwrap());
}