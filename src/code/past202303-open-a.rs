// https://atcoder.jp/contests/past202303-open/tasks/past202303_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        T: usize,
    }
    if (N + T) % 2 == S {
        println!("Bob");
    }
    else {
        println!("Alice");
    }
}