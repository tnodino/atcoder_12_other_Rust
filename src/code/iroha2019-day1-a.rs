// https://atcoder.jp/contests/iroha2019-day1/tasks/iroha2019_day1_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    println!("{}", &S[..=0]);
}