// https://atcoder.jp/contests/dwacon2017-prelims/tasks/dwango2017qual_a

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
    }
    println!("{}", max(0, a + b - n));
}