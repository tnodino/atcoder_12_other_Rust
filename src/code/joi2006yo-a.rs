// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut A = 0;
    let mut B = 0;
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        if a == b {
            A += a;
            B += b;
        }
        else if a > b {
            A += a + b;
        }
        else {
            B += a + b;
        }
    }
    println!("{} {}", A, B);
}