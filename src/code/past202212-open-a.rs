// https://atcoder.jp/contests/past202212-open/tasks/past202212_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
    }
    println!("{}", N / X * Y);
}