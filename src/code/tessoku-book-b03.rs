// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cb

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                if A[i] + A[j] + A[k] == 1_000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}