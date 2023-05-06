// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut ans = 0;
    for i in 1..=N {
        for j in 1..=N {
            if i + j < K && K - (i + j) <= N {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}