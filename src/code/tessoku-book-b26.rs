// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cy

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut sieve = vec![true; N+1];
    for i in 2..=N {
        if sieve[i] {
            println!("{}", i);
            for k in (i..=N).step_by(i) {
                sieve[k] = false;
            }
        }
    }
}