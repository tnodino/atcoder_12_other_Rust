// https://atcoder.jp/contests/gigacode-2019/tasks/gigacode_2019_c

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        mut a: [usize; D],
        b: [usize; D],
    }
    for i in 1..D {
        a[i] += a[i-1];
    }
    let mut ans = INF;
    for i in 0..D {
        if a[i] >= b[i] {
            ans = min(ans, b[i]);
        }
    }
    if ans == INF {
        println!("-1");
    }
    else {
        println!("{}", ans);
    }
}