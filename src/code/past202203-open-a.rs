// https://atcoder.jp/contests/past202203-open/tasks/past202203_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
    }
    let arr = [A * B, A * C, B * C];
    println!("{} {}", arr.iter().min().unwrap(), arr.iter().max().unwrap());

}