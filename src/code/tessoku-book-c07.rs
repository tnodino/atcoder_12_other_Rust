// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fe

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut C: [usize; N],
    }
    C.sort();
    for i in 0..N-1 {
        C[i+1] += C[i];
    }
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            X: usize,
        }
        println!("{}", match C.binary_search(&X) {
            Ok(idx) => idx + 1,
            Err(idx) => idx,
        });
    }
}