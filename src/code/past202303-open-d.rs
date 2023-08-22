// https://atcoder.jp/contests/past202303-open/tasks/past202303_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        A: usize,
        B: usize,
        C: usize,
        D: usize,
    }
    let mut ans = 1<<60;
    for i in 0..=30 {
        let mut res = 0;
        let mut h = H;
        for _ in 0..i {
            if h == 0 {
                break;
            }
            res += D;
            if h <= C {
                h = 0;
                break;
            }
            h -= C;
            h -= h / 2;
        }
        ans = min(ans, res + (h + A - 1) / A * B);
    }
    println!("{}", ans);
}