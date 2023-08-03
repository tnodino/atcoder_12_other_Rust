// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ak

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        B: usize,
        A: [usize; N],
        C: [usize; M],
    }
    println!("{}", A.iter().sum::<usize>() * M + B * N * M + C.iter().sum::<usize>() * N);
}