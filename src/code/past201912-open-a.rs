// https://atcoder.jp/contests/past201912-open/tasks/past201912_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    match S.parse::<usize>() {
        Ok(v) => println!("{}", v * 2),
        Err(_) => println!("error"),
    }
}