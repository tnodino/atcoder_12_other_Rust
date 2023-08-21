// https://atcoder.jp/contests/pakencamp-2020-day1/tasks/pakencamp_2020_day1_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
    }
    let mut ans = 0;
    while X > 0 {
        ans = max(ans, X % 10);
        X /= 10;
    }
    println!("{}", ans);
}