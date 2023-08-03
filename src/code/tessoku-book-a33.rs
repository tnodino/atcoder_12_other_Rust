// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ag

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut nim = 0;
    for i in 0..N {
        nim ^= A[i];
    }
    if nim == 0 {
        println!("Second");
    }
    else {
        println!("First");
    }
}