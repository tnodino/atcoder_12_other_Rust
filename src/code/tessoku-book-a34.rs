// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ah

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        A: [usize; N],
    }
    let M = 100_000;
    let mut grundy = vec![0; M+1];
    for i in 1..=M {
        let mut set = HashSet::new();
        if X <= i {
            set.insert(grundy[i-X]);
        }
        if Y <= i {
            set.insert(grundy[i-Y]);
        }
        for s in 0..10 {
            if !set.contains(&s) {
                grundy[i] = s;
                break;
            }
        }
    }
    let mut nim = 0;
    for i in 0..N {
        nim ^= grundy[A[i]];
    }
    if nim == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }
}