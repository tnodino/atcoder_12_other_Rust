// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_aj

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let M = N * 2 - 2;
    if M > K || M % 2 != K % 2 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}