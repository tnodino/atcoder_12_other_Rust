// https://atcoder.jp/contests/abc150/tasks/abc150_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        K: usize,
        X: usize,
    }
    if K * 500 >= X {
        println!("Yes");
    }
    else {
        println!("No");
    }
}