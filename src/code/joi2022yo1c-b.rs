// https://atcoder.jp/contests/joi2022yo1c/tasks/joi2022_yo1c_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        mut A: usize,
        B: usize,
    }
    let mut ans = 250;
    while A < S {
        A += B;
        ans += 100;
    }
    println!("{}", ans);
}