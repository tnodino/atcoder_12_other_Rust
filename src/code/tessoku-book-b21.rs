// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ct

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut DP = vec![vec![0; N]; N];
    for s in 0..N {
        for l in 0..N {
            let r = l + s;
            if N <= r {
                break;
            }
            if l == r {
                DP[l][r] = 1;
                continue;
            }
            if l + 1 == r && S[l] == S[r] {
                DP[l][r] = 2;
                continue;
            }
            DP[l][r] = max(DP[l+1][r], DP[l][r-1]);
            if S[l] == S[r] {
                DP[l][r] = max(DP[l][r], DP[l+1][r-1] + 2);
            }
            else {
                DP[l][r] = max(DP[l][r], DP[l+1][r-1]);
            }
        }
    }
    println!("{}", DP[0][N-1]);
}