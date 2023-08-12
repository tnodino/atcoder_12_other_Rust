// https://atcoder.jp/contests/dp/tasks/dp_n

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut s = vec![vec![0; N]; N];
    for i in 0..N {
        let mut res = 0;
        for j in i..N {
            res += a[j];
            s[i][j] = res;
        }
    }
    let mut DP = vec![vec![0; N]; N];
    for i in 1..N {
        for l in 0..N {
            let r = l + i;
            if r >= N {
                break;
            }
            let mut res = 1<<60;
            for m in l..r {
                res = min(res, DP[l][m] + DP[m+1][r]);
            }
            DP[l][r] = res + s[l][r];
        }
    }
    println!("{}", DP[0][N-1]);
}