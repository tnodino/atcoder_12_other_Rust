// https://atcoder.jp/contests/APG4b/tasks/APG4b_cj

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [isize; N],
    }
    let su = A.iter().sum::<isize>();
    let M = su / (N as isize);
    for i in 0..N {
        println!("{}", (A[i] - M).abs());
    }
}