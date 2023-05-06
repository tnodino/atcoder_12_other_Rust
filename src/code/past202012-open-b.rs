// https://atcoder.jp/contests/past202012-open/tasks/past202012_b

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
    let mut T = Vec::new();
    for i in (0..N).rev() {
        if T.contains(&S[i]) {
            continue;
        }
        T.push(S[i]);
    }
    println!("{}", T.iter().rev().map(|&x| x.to_string()).collect::<String>());
}