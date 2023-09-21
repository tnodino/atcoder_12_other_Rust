// https://atcoder.jp/contests/practice2/tasks/practice2_c

use proconio::input;
use proconio::fastout;
use ac_library::floor_sum;

#[allow(non_snake_case)]
fn solve() {
    input! {
        (N, M, A, B): (i64, i64, i64, i64),
    }
    println!("{}", floor_sum(N, M, A, B));
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
    }
    for _ in 0..T {
        solve();
    }
}