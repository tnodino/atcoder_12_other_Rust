// https://atcoder.jp/contests/past202212-open/tasks/past202212_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: f64,
        B: f64,
        C: f64,
        D: f64,
    }
    if A / B < C / D {
        println!("<");
    }
    else if A / B > C / D {
        println!(">");
    }
    else {
        println!("=");
    }
}