// https://atcoder.jp/contests/APG4b/tasks/APG4b_ch

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
    }
    println!("{}", A.iter().sum::<usize>() * B.iter().sum::<usize>() * C.iter().sum::<usize>());
}