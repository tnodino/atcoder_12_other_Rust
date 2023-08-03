// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut M = 1000 - N;
    let vec = [500, 100, 50, 10, 5, 1];
    let mut ans = 0;
    for i in 0..6 {
        ans += M / vec[i];
        M %= vec[i];
    }
    println!("{}", ans);
}