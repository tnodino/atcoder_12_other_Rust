// https://atcoder.jp/contests/codequeen2023-final-open/tasks/codequeen2023_final_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        c: char,
    }
    let N = S.len();
    let S = S.chars().rev().collect::<Vec<char>>();
    let mut T = Vec::new();
    for i in (0..N).rev() {
        if S[i] == c {
            T.push(c);
        }
        T.push(S[i]);
    }
    println!("{}", T.iter().collect::<String>());
}