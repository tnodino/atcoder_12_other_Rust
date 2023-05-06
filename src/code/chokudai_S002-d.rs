// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans: usize = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        ans += max(A, B);
    }
    println!("{}", ans);
}