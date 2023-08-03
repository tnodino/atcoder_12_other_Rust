// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_aq

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: char,
        }
        if B == 'E' {
            ans = max(ans, L - A);
        }
        else {
            ans = max(ans, A);
        }
    }
    println!("{}", ans);
}