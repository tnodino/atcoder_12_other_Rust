// https://atcoder.jp/contests/abc299/tasks/abc299_a

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
    let mut s = Vec::new();
    let mut t = Vec::new();
    for i in 0..N {
        if S[i] == '|' {
            s.push(i);
        }
        if S[i] == '*' {
            t.push(i);
        }
    }
    if s[0] < t[0] && t[0] < s[1] {
        println!("in");
    }
    else {
        println!("out");
    }
}