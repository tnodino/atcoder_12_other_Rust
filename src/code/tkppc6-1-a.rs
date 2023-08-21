// https://atcoder.jp/contests/tkppc6-1/editorial

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N <= 2014 || N == 2017 {
        println!("-1")
    }
    else if N <= 2016 {
        println!("{}", N - 2014);
    }
    else {
        println!("{}", N - 2015);
    }
}