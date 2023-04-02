// https://atcoder.jp/contests/abc110/tasks/abc110_a

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
    arr.sort_by(|a, b| b.cmp(a));
    println!("{}", arr[0] * 10 + arr[1] + arr[2]);
}