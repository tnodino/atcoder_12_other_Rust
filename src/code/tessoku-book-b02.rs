// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ca

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let mut vec = Vec::new();
    for i in 1..=100 {
        if 100 % i == 0 {
            vec.push(i);
        }
    }
    for v in vec {
        if A <= v && v <= B {
            println!("Yes");
            return;
        }
    }
    println!("No");
}