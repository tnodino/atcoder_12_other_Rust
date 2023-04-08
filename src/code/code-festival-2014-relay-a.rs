// https://atcoder.jp/contests/code-festival-2014-relay/tasks/code_festival_relay_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    if n == 1 {
        println!("0");
    }
    else {
        println!("1");
    }
}