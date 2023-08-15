// https://atcoder.jp/contests/joisc2007/tasks/joisc2007_mall

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: isize = 1<<32;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        m: usize,
        n: usize,
        b: usize,
        a: usize,
        c: [[isize; m]; n],
    }
    let mut cost = vec![vec![0; m+1]; n+1];
    for i in 1..=n {
        for j in 1..=m {
            cost[i][j] = cost[i-1][j] + cost[i][j-1] - cost[i-1][j-1];
            if c[i-1][j-1] == -1 {
                cost[i][j] += INF;
            }
            else {
                cost[i][j] += c[i-1][j-1];
            }
        }
    }
    let mut ans = INF;
    for i in 0..=n-a {
        for j in 0..=m-b {
            let c = cost[i+a][j+b] + cost[i][j] - cost[i+a][j] - cost[i][j+b];
            ans = min(ans, c);
        }
    }
    println!("{}", ans);
}