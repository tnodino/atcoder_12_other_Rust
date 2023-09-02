// https://atcoder.jp/contests/soundhound2018/tasks/soundhound2018_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: String,
        Y: String,
    }
    if &X[..=0] == "S" && &Y[..=0] == "H" {
        println!("YES");
    }
    else {
        println!("NO");
    }
}