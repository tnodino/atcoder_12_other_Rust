// https://atcoder.jp/contests/arc014/tasks/arc014_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N % 2 == 0 {
        println!("Blue");
    }
    else {
        println!("Red");
    }
}