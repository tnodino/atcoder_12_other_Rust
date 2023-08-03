// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ch

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1500;
    let mut S = vec![vec![0; M+10]; M+10];
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize,
        }
        S[A][B] += 1;
        S[A][D] -= 1;
        S[C][B] -= 1;
        S[C][D] += 1;
    }
    for i in 0..=M {
        for j in 0..=M {
            S[i+1][j] += S[i][j];
        }
    }
    for i in 0..=M {
        for j in 0..=M {
            S[i][j+1] += S[i][j];
        }
    }
    let mut ans = 0;
    for i in 0..=M {
        for j in 0..=M {
            if S[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}