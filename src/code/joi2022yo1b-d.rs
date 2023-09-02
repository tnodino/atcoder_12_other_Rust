// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_d

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 2000;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    let mut mi = N;
    for i in 1..=M {
        if cnt[i] > 0 {
            mi = min(mi, cnt[i]);
        }
    }
    for i in 1..=M {
        if cnt[i] == mi {
            println!("{}", i);
            break;
        }
    }
}