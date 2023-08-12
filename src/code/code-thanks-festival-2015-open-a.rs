// https://atcoder.jp/contests/code-thanks-festival-2015-open/tasks/code_thanks_festival_2015_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
    }
    println!("{}", A.abs() + (A - B).abs() + B.abs());
}