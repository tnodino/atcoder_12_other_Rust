// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_u

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut P = Vec::new();
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            p: usize,
            a: usize,
        }
        P.push(p-1);
        A.push(a);
    }
    let mut DP = vec![vec![0; N]; N];
    for s in 1..N {
        for l in 0..N {
            let r = l + s;
            if N <= r {
                break;
            }
            let res1;
            if l <= P[l] && P[l] <= r {
                res1 = A[l];
            }
            else {
                res1 = 0;
            }
            let res2;
            if l <= P[r] && P[r] <= r {
                res2 = A[r];
            }
            else {
                res2 = 0;
            }
            DP[l][r] = max(DP[l+1][r] + res1, DP[l][r-1] + res2);
        }
    }
    println!("{}", DP[0][N-1]);
}