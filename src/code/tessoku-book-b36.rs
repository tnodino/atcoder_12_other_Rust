// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_di

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        K: usize,
        S: String,
    }
    let M = S.chars().filter(|&x| x == '1').count();
    if M % 2 == K % 2 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}