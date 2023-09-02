// https://atcoder.jp/contests/joi2022yo1b/tasks/joi2022_yo1b_c

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
    for i in 0..N-1 {
        if S[i+1] == 'J' {
            println!("{}", S[i]);
        }
    }
}