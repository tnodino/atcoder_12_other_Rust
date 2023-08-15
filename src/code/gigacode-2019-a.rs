// https://atcoder.jp/contests/gigacode-2019/tasks/gigacode_2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    println!("{}", A * (B * B));
}