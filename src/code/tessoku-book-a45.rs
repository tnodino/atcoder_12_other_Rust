// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_as

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        C: char,
        A: String,
    }
    let mut s = 0;
    for a in A.chars() {
        s += match a {
            'W' => 0,
            'B' => 1,
            _ => 2,
        };
        s %= 3;
    }
    if s == match C {
        'W' => 0,
        'B' => 1,
        _ => 2,
    } {
        println!("Yes");
    }
    else {
        println!("No");
    }
}