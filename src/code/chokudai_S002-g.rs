// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_g

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        println!("{}", gcd(A, B));
    }
}