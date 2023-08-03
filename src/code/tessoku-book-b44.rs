// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dq

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [[usize; N]; N],
        Q: usize,
    }
    let mut idx = (0..N).collect::<Vec<usize>>();
    for _ in 0..Q {
        input! {
            q: usize,
            x: usize,
            y: usize,
        }
        if q == 1 {
            idx.swap(x-1, y-1);
        }
        else {
            println!("{}", A[idx[x-1]][y-1]);
        }
    }
}