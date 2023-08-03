// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cl

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let mut s = vec![0; N+1];
    for i in 0..N {
        s[i+1] += s[i] + A[i];
    }
    let mut l = 0;
    let mut ans = 0;
    for r in 1..=N {
        while s[r] - s[l] > K{
            l += 1;
        }
        ans += r - l;
    }
    println!("{}", ans);
}