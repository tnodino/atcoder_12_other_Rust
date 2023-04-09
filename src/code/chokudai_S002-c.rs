// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        ans = max(ans, A + B);
    }
    println!("{}", ans);
}