// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut S = S.chars().collect::<Vec<char>>();
    S.sort();
    S.dedup();
    if S.len() >= 3 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}