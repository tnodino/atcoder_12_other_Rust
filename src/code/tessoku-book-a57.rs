// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_be

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [usize; N],
    }
    let mut DP = vec![vec![0; N]; 64];
    for i in 0..N {
        DP[0][i] = A[i] - 1;
    }
    for d in 1..64 {
        for i in 0..N {
            DP[d][i] = DP[d-1][DP[d-1][i]];
        }
    }
    for _ in 0..Q {
        input! {
            X: usize,
            Y: usize,
        }
        let mut pos = X - 1;
        for d in (0..64).rev() {
            if Y & (1 << d) > 0 {
                pos = DP[d][pos];
            }
        }
        println!("{}", pos + 1);
    }
}