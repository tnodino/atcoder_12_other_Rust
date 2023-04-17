// https://atcoder.jp/contests/abc298/tasks/abc298_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    if S.contains("o") && !S.contains("x") {
        println!("Yes");
    }
    else {
        println!("No");
    }
}