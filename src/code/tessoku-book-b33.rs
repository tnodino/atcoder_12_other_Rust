// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_df

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        _H: usize,
        _W: usize,
    }
    let mut nim = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        nim ^= A - 1;
        nim ^= B - 1;
    }
    if nim == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }
}