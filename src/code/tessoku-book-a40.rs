// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_an

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let M = 100;
    let mut cnt = vec![0; M+1];
    for i in 0..N {
        cnt[A[i]] += 1;
    }
    let mut ans: usize = 0;
    for i in 1..=M {
        if cnt[i] >= 3 {
            ans += cnt[i] * (cnt[i] - 1) * (cnt[i] - 2) / 6;
        }
    }
    println!("{}", ans);
}