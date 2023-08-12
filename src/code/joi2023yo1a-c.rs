// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_c

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut now = 1;
    let mut ans = 0;
    for i in 0..N {
        now = match S[i] {
            'L' => max(1, now-1),
            _ => min(3, now+1),
        };
        if now == 3 {
            ans += 1;
        }
    }
    println!("{}", ans);
}