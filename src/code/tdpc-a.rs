// https://atcoder.jp/contests/tdpc/tasks/tdpc_contest

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N],
    }
    let M = 10_000;
    let mut DP = vec![vec![false; M+1]; N+1];
    DP[0][0] = true;
    for i in 0..N {
        for j in 0..=M {
            DP[i+1][j] |= DP[i][j];
            if j + p[i] <= M {
                DP[i+1][j+p[i]] |= DP[i][j];
            }
        }
    }
    println!("{}", DP[N].iter().filter(|&x| *x).count());
}