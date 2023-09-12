// https://atcoder.jp/contests/past201912-open/tasks/past201912_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (A, B, C, D, E, F): (usize, usize, usize, usize, usize, usize),
    }
    let mut vec = vec![A, B, C, D, E, F];
    vec.sort();
    println!("{}", vec[3]);
}