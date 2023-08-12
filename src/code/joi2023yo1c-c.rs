// https://atcoder.jp/contests/joi2023yo1c/tasks/joi2023_yo1c_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    for i in 0..N-1 {
        if S[i] == S[i+1] {
            S[i] = S[i].to_ascii_uppercase();
            S[i+1] = S[i+1].to_ascii_uppercase();
        }
    }
    println!("{}", S.iter().collect::<String>());
}