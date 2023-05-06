// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fb

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut vec: Vec<usize> = Vec::new();
    for i in 1..=M {
        if N % i == 0 {
            vec.push(i);
            vec.push(N / i);
        }
    }
    vec.sort();
    vec.dedup();
    for v in vec {
        println!("{}", v);
    }
}