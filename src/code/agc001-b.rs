// https://atcoder.jp/contests/agc001/tasks/agc001_b

use proconio::input;
use proconio::fastout;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
    }
    let mut a = X;
    let mut b = N - X;
    if a < b {
        swap(&mut a, &mut b);
    }
    let mut ans = N;
    while b > 0 {
        ans += a / b * b * 2;
        a %= b;
        swap(&mut a, &mut b);
    }
    println!("{}", ans - a);
}