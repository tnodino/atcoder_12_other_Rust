// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    println!("{}", S.replace("joi", "JOI"));
}