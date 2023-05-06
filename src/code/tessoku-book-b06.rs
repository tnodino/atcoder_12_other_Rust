// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ce

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        Q: usize,
    }
    let mut B = vec![0; N+1];
    for i in 0..N {
        B[i+1] += A[i] + B[i];
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
        }
        let win = B[R] - B[L-1];
        let lose = (R - L) - win + 1;
        if win > lose {
            println!("win");
        }
        else if win < lose {
            println!("lose");
        }
        else {
            println!("draw");
        }
    }
}