// https://atcoder.jp/contests/joi2007ho/tasks/joi2007ho_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [isize; n],
    }
    let mut s = 0;
    for i in 0..k {
        s += a[i];
    }
    let mut ans = s;
    for i in k..n {
        s += a[i];
        s -= a[i-k];
        ans = max(ans, s);
    }
    println!("{}", ans);
}