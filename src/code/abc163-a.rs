// https://atcoder.jp/contests/abc163/tasks/abc163_a

use proconio::input;
use proconio::fastout;
use std::f64::consts::PI;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        R: f64,
    }
    println!("{}", R * 2. * PI);
}