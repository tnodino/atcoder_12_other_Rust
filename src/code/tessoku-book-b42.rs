// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_do

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: isize,
            b: isize,
        }
        A.push(a);
        B.push(b);
    }
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if A[i] + B[i] > 0 {
            x += A[i];
            y += B[i];
        }
    }
    ans = max(ans, x.abs() + y.abs());
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if -A[i] + B[i] > 0 {
            x += A[i];
            y += B[i];
        }
    }
    ans = max(ans, x.abs() + y.abs());
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if A[i] + -B[i] > 0 {
            x += A[i];
            y += B[i];
        }
    }
    ans = max(ans, x.abs() + y.abs());
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if -A[i] + -B[i] > 0 {
            x += A[i];
            y += B[i];
        }
    }
    ans = max(ans, x.abs() + y.abs());
    println!("{}", ans);
}