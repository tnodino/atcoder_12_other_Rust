// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ap

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut ans = 0;
    for al in 1..=100 {
        let ar = al + K;
        for bl in 1..=100 {
            let br = bl + K;
            let mut cnt = 0;
            for i in 0..N {
                if al <= A[i] && A[i] <= ar && bl <= B[i] && B[i] <= br {
                    cnt += 1;
                }
            }
            ans = max(ans, cnt);
        }
    }
    println!("{}", ans);
}