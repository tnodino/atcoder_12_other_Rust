// https://atcoder.jp/contests/abc105/tasks/abc105_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    if N % K == 0 {
        println!("0");
    }
    else {
        println!("1");
    }
}