// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let M = N / 2;
    if S[..M] == S[M..] {
        println!("Yes");
    }
    else {
        println!("No");
    }
}