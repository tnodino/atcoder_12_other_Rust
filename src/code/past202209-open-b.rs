// https://atcoder.jp/contests/past202209-open/tasks/past202209_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: String,
    }
    if N.len() <= 2 {
        println!("0");
    }
    else {
        println!("{}", &N[..N.len()-2]);
    }
}