// https://atcoder.jp/contests/utpc2011/tasks/utpc2011_1

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        M: usize,
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..M {
        input! {
            a: [usize; N],
        }
        ans = max(ans, a.iter().sum::<usize>());
    }
    println!("{}", ans);
}