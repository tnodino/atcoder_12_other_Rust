// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dk

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut lim = vec![1; N];
    for i in 0..N-1 {
        if S[i] == 'A' {
            if lim[i] >= lim[i+1] {
                lim[i+1] = lim[i] + 1;
            }
        }
    }
    for i in (0..N-1).rev() {
        if S[i] == 'B' {
            if lim[i+1] >= lim[i] {
                lim[i] = lim[i+1] + 1;
            }
        }
    }
    println!("{}", lim.iter().sum::<usize>());
}