// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cg

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1500;
    let mut S = vec![vec![0; M+1]; M+1];
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
        }
        S[X][Y] += 1;
    }
    for i in 0..M {
        for j in 0..=M {
            S[i+1][j] += S[i][j];
        }
    }
    for i in 0..=M {
        for j in 0..M {
            S[i][j+1] += S[i][j];
        }
    }
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        println!("{}", S[c][d] + S[a-1][b-1] - S[a-1][d] - S[c][b-1]);
    }
}