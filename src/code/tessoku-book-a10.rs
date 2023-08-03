// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_j

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        D: usize,
    }
    let mut L_max = vec![0; N+1];
    let mut R_max = vec![0; N+1];
    for i in 0..N {
        L_max[i+1] = max(L_max[i], A[i]);
    }
    for i in (0..N).rev() {
        R_max[i] = max(R_max[i+1], A[i]);
    }
    for _ in 0..D {
        input! {
            L: usize,
            R: usize,
        }
        println!("{}", max(L_max[L-1], R_max[R]));
    }
}