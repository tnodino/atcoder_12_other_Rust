// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 1;
    let mut l = 0;
    for r in 1..N {
        if A[r-1] > A[r] {
            l = r;
        }
        ans = max(ans, r + 1 - l);
    }
    println!("{}", ans);
}