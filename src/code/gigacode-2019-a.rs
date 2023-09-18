// https://atcoder.jp/contests/gigacode-2019/tasks/gigacode_2019_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B): (usize, usize),
    }
    println!("{}", B * B * A);
}