// https://atcoder.jp/contests/pakencamp-2021-day2/tasks/pakencamp_2021_day2_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
    }
    println!("{}", a * a * b);
}