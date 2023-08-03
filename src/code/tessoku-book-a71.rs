// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bs

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N],
    }
    A.sort();
    B.sort_by(|a, b| b.cmp(a));
    let mut ans = 0;
    for i in 0..N {
        ans += A[i] * B[i];
    }
    println!("{}", ans);
}