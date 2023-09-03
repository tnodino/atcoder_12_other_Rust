// https://atcoder.jp/contests/bitflyer2018-qual/tasks/bitflyer2018_qual_b

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: isize,
        mut B: isize,
        _N: usize,
        X: String,
    }
    for x in X.chars() {
        match x {
            'S' => A -= 1,
            'C' => B -= 1,
            _ => {
                if A >= B {
                    A -= 1;
                }
                else {
                    B -= 1;
                }
            },
        }
    }
    println!("{}", max(0, A));
    println!("{}", max(0, B));
}