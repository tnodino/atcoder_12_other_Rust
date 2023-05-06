// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fd

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", N);
    println!("{} {}", 1, N);
    for i in 1..N {
        println!("{} {}", i, i + 1);
    }
}