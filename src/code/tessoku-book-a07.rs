// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_g

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        N: usize,
    }
    let mut imos = vec![0; D+1];
    for _ in 0..N {
        input! {
            L: usize,
            R: usize,
        }
        imos[L-1] += 1;
        imos[R] -= 1;
    }
    for i in 0..D {
        println!("{}", imos[i]);
        imos[i+1] += imos[i];
    }
}