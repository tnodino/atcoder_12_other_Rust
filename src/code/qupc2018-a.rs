// https://atcoder.jp/contests/qupc2018/tasks/qupc2018_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", 2010 + N * 4);
}