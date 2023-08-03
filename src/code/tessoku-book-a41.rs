// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ao

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
    for i in 0..N-2 {
        if S[i] == S[i+1] && S[i+1] == S[i+2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}