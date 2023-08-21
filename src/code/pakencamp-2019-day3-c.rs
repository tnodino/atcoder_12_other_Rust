// https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [[usize; M]; N],
    }
    let mut ans = 0;
    for a in 0..M {
        for b in a+1..M {
            let mut res = 0;
            for i in 0..N {
                res += max(A[i][a], A[i][b])
            }
            ans = max(ans, res);
        }
    }
    println!("{}", ans);
}