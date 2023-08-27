// https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut r = vec![0; N];
    let mut b = vec![0; N];
    let mut w = vec![0; N];
    for _ in 0..5 {
        input! {
            S: String,
        }
        let S = S.chars().collect::<Vec<char>>();
        for i in 0..N {
            match S[i] {
                'R' => r[i] += 1,
                'B' => b[i] += 1,
                'W' => w[i] += 1,
                _ => continue,
            }
        }
    }
    let mut DP = vec![vec![INF; 3]; N+1];
    DP[0] = vec![0; 3];
    for i in 0..N {
        DP[i+1][0] = min(DP[i][1], DP[i][2]) + (5 - r[i]);
        DP[i+1][1] = min(DP[i][0], DP[i][2]) + (5 - b[i]);
        DP[i+1][2] = min(DP[i][0], DP[i][1]) + (5 - w[i]);
    }
    println!("{}", DP[N].iter().min().unwrap());
}