// https://atcoder.jp/contests/APG4b/tasks/APG4b_cr

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A + B);
}