// https://atcoder.jp/contests/dp/tasks/dp_o

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

fn popcount(mut x: usize) -> usize {
    let mut ret = 0;
    while x > 0 {
        if x & 1 > 0 {
            ret += 1;
        }
        x >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [[usize; N]; N],
    }
    let mut DP = vec![0; 1<<N];
    DP[0] = 1;
    for bit in 1..1<<N {
        let i = popcount(bit);
        for j in 0..N {
            if bit & (1 << j) > 0 && a[i-1][j] == 1 {
                DP[bit] += DP[bit ^ (1 << j)];
                DP[bit] %= MOD;
            }
        }
    }
    println!("{}", DP[(1 << N) - 1]);
}