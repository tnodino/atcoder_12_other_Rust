// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dr

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    if a + b + c == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}