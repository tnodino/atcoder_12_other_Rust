// https://atcoder.jp/contests/APG4b/tasks/APG4b_ci

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
    let mut arr = [A, B, C];
    arr.sort();
    println!("{}", arr[2] - arr[0]);
}