// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    for i in 0..N {
        if A[i] == X {
            println!("Yes");
            return;
        }
    }
    println!("No");
}