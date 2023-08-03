// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_m

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
    let mut l = 0;
    let mut ans = 0;
    for r in 0..N {
        while A[r] - A[l] > K {
            l += 1;
        }
        ans += r - l;
    }
    println!("{}", ans);
}