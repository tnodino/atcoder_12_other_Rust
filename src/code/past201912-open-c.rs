// https://atcoder.jp/contests/past201912-open/tasks/past201912_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: usize,
        F: usize,
    }
    let mut arr = [A, B, C, D, E, F];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{}", arr[2]);
}