// https://atcoder.jp/contests/abc126/tasks/abc126_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        K: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S[K-1] = (S[K-1] as u8 + 32) as char;
    println!("{}", S.iter().collect::<String>());
}