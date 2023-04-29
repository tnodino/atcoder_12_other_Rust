// https://atcoder.jp/contests/past202104-open/tasks/past202104_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in 0..8 {
        if i != 3 && S[i] == '-' {
            println!("No");
            return;
        }
        if i == 3 && S[i] != '-' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}