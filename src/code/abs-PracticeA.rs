// https://atcoder.jp/contests/abs/tasks/practice_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        s: String,
    }
    println!("{} {}", a + b + c, s);
}