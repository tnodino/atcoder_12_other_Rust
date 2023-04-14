// https://atcoder.jp/contests/abc105/tasks/abc105_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for a in 0..=30 {
        for b in 0..=30 {
            if a * 4 + b * 7 == N {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}