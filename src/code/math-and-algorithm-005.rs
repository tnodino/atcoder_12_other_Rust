// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_e

use proconio::input;
use proconio::fastout;

const MOD: usize = 100;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += a[i];
        ans %= MOD;
    }
    println!("{}", ans);
}