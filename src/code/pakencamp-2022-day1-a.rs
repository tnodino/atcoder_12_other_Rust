// https://atcoder.jp/contests/pakencamp-2022-day1/tasks/pakencamp_2022_day1_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    if X == 0 && Y == 0 {
        println!("0");
    }
    else if X == 0 || Y == 0 {
        println!("1");
    }
    else {
        println!("2");
    }
}