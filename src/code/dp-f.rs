// https://atcoder.jp/contests/dp/tasks/dp_f

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let N = s.len();
    let M = t.len();
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![0; M+1]; N+1];
    for i in 1..=N {
        for j in 1..=M {
            DP[i][j] = max(DP[i-1][j], DP[i][j-1]);
            if s[i-1] == t[j-1] {
                DP[i][j] = max(DP[i][j], DP[i-1][j-1] + 1);
            }
        }
    }
    let mut x = N;
    let mut y = M;
    let mut ans = "".to_string();
    while x > 0 && y > 0 {
        if s[x-1] == t[y-1] {
            ans = format!("{}{}", ans, s[x-1]);
            x -= 1;
            y -= 1;
        }
        else if DP[x][y] == DP[x-1][y] {
            x -= 1;
        }
        else {
            y -= 1;
        }
    }
    println!("{}", ans.chars().rev().collect::<String>());
}