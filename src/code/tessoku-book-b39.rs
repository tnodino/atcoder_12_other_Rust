// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dl

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
    }
    let mut vec = vec![Vec::new(); D];
    for _ in 0..N {
        input! {
            X: usize,
            Y: usize,
        }
        vec[X-1].push(Y);
    }
    let mut bh = BinaryHeap::new();
    let mut ans = 0;
    for d in 0..D {
        for i in 0..vec[d].len() {
            bh.push(vec[d][i]);
        }
        if !bh.is_empty() {
            ans += bh.pop().unwrap();
        }
    }
    println!("{}", ans);
}