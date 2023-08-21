// https://atcoder.jp/contests/typical90/tasks/typical90_p

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        B: usize,
        C: usize,
    }
    let mut ans = 9999;
    for x in 0..=9999 {
        for y in 0..=9999 {
            let m = A * x + B * y;
            if m > N {
                break;
            }
            if (N - m) % C != 0 {
                continue;
            }
            let z = (N - m) / C;
            ans = min(ans, x + y + z);
        }
    }
    println!("{}", ans);
}