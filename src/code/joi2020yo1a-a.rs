// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    if A == B || A == C {
        println!("{}", A);
    }
    else {
        println!("{}", B);
    }
}