// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dg

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _X: usize,
        _Y: usize,
        A: [usize; N],
    }
    let mut nim = 0;
    for i in 0..N {
        let m = A[i] % 5;
        if m <= 1 {
            continue;
        }
        if m <= 3 {
            nim ^= 1;
        }
        else {
            nim ^= 2;
        }
    }
    if nim == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }
}