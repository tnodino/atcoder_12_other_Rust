// https://atcoder.jp/contests/past202005-open/tasks/past202005_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
        t: String,
    }
    if s == t {
        println!("same");
    }
    else if s.to_lowercase() == t.to_lowercase() {
        println!("case-insensitive");
    }
    else {
        println!("different");
    }
}