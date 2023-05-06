// https://atcoder.jp/contests/joi2023yo2/tasks/joi2023_yo2_a

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
    let mi = A.iter().min().unwrap();
    let ma = A.iter().max().unwrap();
    for i in 0..N {
        println!("{}", max(ma - A[i], A[i] - mi));
    }
}