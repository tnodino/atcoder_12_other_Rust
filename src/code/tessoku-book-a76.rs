// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bx

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: isize,
        L: isize,
        R: isize,
        x: [isize; N],
    }
    let M = N + 1;
    let mut X = Vec::new();
    X.push(0);
    X.extend(x);
    X.push(W);
    let mut DP = vec![0; M+1];
    let mut S = vec![0; M+1];
    DP[0] = 1;
    S[0] = 1;
    for i in 1..=M {
        let l_idx = bisect_left(&X, &(X[i]-R));
        let r_idx = bisect_left(&X, &(X[i]-L+1));
        if r_idx >= 1 {
            DP[i] = S[r_idx-1];
        }
        if l_idx >= 1 {
            DP[i] = (DP[i] + MOD - S[l_idx-1]) % MOD;
        }
        S[i] = S[i-1] + DP[i];
        S[i] %= MOD;
    }
    println!("{}", DP[M]);
}