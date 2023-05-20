// https://atcoder.jp/contests/past202107-open/tasks/past202107_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        L: String,
        R: String,
    }
    if S.len() >= 2 && S.chars().next().unwrap() == '0' {
        println!("No");
    }
    else if S.len() < L.len() || R.len() < S.len() {
        println!("No");
    }
    else {
        let S = S.parse::<usize>().unwrap();
        let L = L.parse::<usize>().unwrap();
        let R = R.parse::<usize>().unwrap();
        if L <= S && S <= R {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}