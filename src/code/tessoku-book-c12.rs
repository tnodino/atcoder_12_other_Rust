// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fj

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut score = vec![vec![0; N+1]; N+1];
    for l in 0..N {
        for r in l+1..=N {
            let mut cnt = 0;
            for i in 0..M {
                if l <= A[i] && B[i] <= r {
                    cnt += 1;
                }
            }
            score[l][r] = cnt;
        }
    }
    let mut DP = vec![vec![-1000; N+1]; K+1];
    DP[0][0] = 0;
    for i in 0..K {
        for l in 0..N {
            for r in l+1..=N {
                DP[i+1][r] = max(DP[i+1][r], DP[i][l] + score[l+1][r]);
            }
        }
    }
    println!("{}", DP[K][N]);
}