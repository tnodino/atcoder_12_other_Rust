// https://atcoder.jp/contests/atc002/tasks/atc002_b

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn bin_power(mut x: usize, mut k: usize, MOD: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        P: usize,
    }
    println!("{}", bin_power(N, P, M));
}