// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_as

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
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
    }
    let mut ans = bin_power(4, N + 1) - 1;
    ans *= bin_power(3, MOD-2);
    ans %= MOD;
    println!("{}", ans);
}