// https://atcoder.jp/contests/typical90/tasks/typical90_al

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

const INF: usize = 1_000_000_000_000_000_000;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
    }
    let c = B / gcd(A, B);
    if c > INF / A {
        println!("Large");
    }
    else {
        println!("{}", A * c);
    }
}