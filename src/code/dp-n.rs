// https://atcoder.jp/contests/dp/tasks/dp_n

use proconio::input;
use proconio::fastout;
use std::cmp::min;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut sum = vec![vec![0; N]; N];
    for i in 0..N {
        sum[i][i] = a[i];
        for j in i+1..N {
            sum[i][j] = sum[i][j-1] + a[j];
        }
    }
    let mut DP = vec![vec![INF; N]; N];
    for i in 0..N {
        DP[i][i] = 0;
    }
    for d in 1..N {
        for l in 0..N {
            let r = l + d;
            if N <= r {
                break;
            }
            for m in l..r {
                DP[l][r] = min(DP[l][r], DP[l][m] + DP[m+1][r]);
            }
            DP[l][r] += sum[l][r];
        }
    }
    println!("{}", DP[0][N-1]);
}