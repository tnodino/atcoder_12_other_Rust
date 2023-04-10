// https://atcoder.jp/contests/abc149/tasks/abc149_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        K: usize,
    }
    if K <= A {
        println!("{} {}", A - K, B);
    }
    else if K <= A + B {
        println!("{} {}", 0, B - (K - A));
    }
    else {
        println!("0 0");
    }
}