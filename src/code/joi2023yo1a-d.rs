// https://atcoder.jp/contests/joi2023yo1a/tasks/joi2023_yo1a_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N*2-1],
    }
    A.sort();
    for i in (0..N*2-2).step_by(2) {
        if A[i] != A[i+1] {
            println!("{}", A[i]);
            return;
        }
    }
    println!("{}", N);
}