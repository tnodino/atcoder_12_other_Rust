// https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let s = a + b + c + d;
    let x = s / 60;
    let y = s % 60;
    println!("{}", x);
    println!("{}", y);
}