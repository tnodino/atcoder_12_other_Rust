// https://atcoder.jp/contests/APG4b/tasks/APG4b_cl

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: isize,
    }
    for i in 1..=N {
        input! {
            op: char,
            B: isize,
        }
        if op == '/' && B == 0 {
            println!("error");
            return;
        }
        A = match op {
            '+' => A + B,
            '-' => A - B,
            '*' => A * B,
            _ => A / B,
        };
        println!("{}:{}", i, A);
    }
}