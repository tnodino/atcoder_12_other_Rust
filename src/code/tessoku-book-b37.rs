// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dj

use proconio::input;
use proconio::fastout;
use num::pow;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut cnt = vec![vec![0; 10]; 15];
    let mut M = 0;
    for i in 0..15 {
        for j in 0..10 {
            cnt[i][j] += N / 10 * pow(10, i);
            if j < N % 10 {
                cnt[i][j] += pow(10, i);
            }
            else if j == N % 10 {
                cnt[i][j] += M + 1;
            }
        }
        M += pow(10, i) * (N % 10);
        N /= 10;
    }
    let mut ans = 0;
    for i in 0..15 {
        for j in 0..10 {
            ans += cnt[i][j] * j;
        }
    }
    println!("{}", ans);
}