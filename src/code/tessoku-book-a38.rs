// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_al

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        N: usize,
    }
    let mut lim = vec![24; D];
    for _ in 0..N {
        input! {
            L: usize,
            R: usize,
            H: usize,
        }
        for i in L-1..R {
            lim[i] = min(lim[i], H);
        }
    }
    println!("{}", lim.iter().sum::<usize>());
}