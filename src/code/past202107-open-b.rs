// https://atcoder.jp/contests/past202107-open/tasks/past202107_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
        C: f64,
    }
    if A <= B * C {
        println!("{}", A / B);
    }
    else {
        println!("{}", B * C / B);
    }
}