// https://atcoder.jp/contests/APG4b/tasks/APG4b_cq

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        op: char,
        B: usize,
    }
    match op {
        '+' => println!("{}", A + B),
        '-' => println!("{}", A - B),
        '*' => println!("{}", A * B),
        '/' if B == 0 => println!("error"),
        '/' => println!("{}", A / B),
        _ => println!("error"),
    }
}