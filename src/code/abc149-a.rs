// https://atcoder.jp/contests/abc149/tasks/abc149_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    println!("{}{}", T, S);
}