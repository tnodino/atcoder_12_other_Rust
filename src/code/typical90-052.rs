// https://atcoder.jp/contests/typical90/tasks/typical90_az

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 1;
    for _ in 0..N {
        input! {
            A: [usize; 6],
        }
        ans *= A.iter().sum::<usize>();
        ans %= MOD;
    }
    println!("{}", ans);
}