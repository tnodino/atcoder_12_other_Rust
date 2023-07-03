// https://atcoder.jp/contests/typical90/tasks/typical90_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize; W]; H],
    }
    let mut h = vec![0; H];
    let mut w = vec![0; W];
    for i in 0..H {
        for j in 0..W {
            h[i] += A[i][j];
            w[j] += A[i][j];
        }
    }
    let mut B = vec![vec![0; W]; H];
    for i in 0..H {
        for j in 0..W {
            B[i][j] = h[i] + w[j] - A[i][j];
        }
        println!("{}", B[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}