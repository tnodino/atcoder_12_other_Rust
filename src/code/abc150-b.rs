// https://atcoder.jp/contests/abc150/tasks/abc150_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let S = S.replace("ABC", "x");
    println!("{}", S.chars().filter(|&x| x == 'x').count());
}