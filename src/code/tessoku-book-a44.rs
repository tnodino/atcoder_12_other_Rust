// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ar

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut A = (1..=N).collect::<Vec<usize>>();
    let mut flg = true;
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: usize,
                y: usize,
            }
            if flg {
                A[x-1] = y;
            }
            else {
                A[N-x] = y;
            }
        }
        else if q == 2 {
            flg ^= true;
        }
        else {
            input! {
                x: usize,
            }
            if flg {
                println!("{}", A[x-1]);
            }
            else {
                println!("{}", A[N-x]);
            }
        }
    }
}