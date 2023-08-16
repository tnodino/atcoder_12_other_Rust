// https://atcoder.jp/contests/typical90/tasks/typical90_v

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let g = gcd(A, gcd(B, C));
    println!("{}", A / g + B / g + C / g - 3);
}