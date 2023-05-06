// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_h

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        X: [[usize; W]; H],
        Q: usize,
    }
    let mut s = vec![vec![0; W+1]; H+1];
    for i in 0..H {
        for j in 0..W {
            s[i+1][j+1] += X[i][j] + s[i+1][j] + s[i][j+1] - s[i][j];
        }
    }
    for _ in 0..Q {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize,
        }
        println!("{}", s[C][D] - s[A-1][D] - s[C][B-1] + s[A-1][B-1]);
    }
}