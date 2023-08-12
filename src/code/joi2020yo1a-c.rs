// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize; N],
        B: [usize; M],
    }
    A.extend(&B);
    A.sort();
    for i in 0..N+M {
        println!("{}", A[i]);
    }
}