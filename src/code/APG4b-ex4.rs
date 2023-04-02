// https://atcoder.jp/contests/APG4b/tasks/APG4b_cs

use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let seconds: usize = 365 * 24 * 60 * 60;
    println!("{}", seconds);
    println!("{}", seconds * 2);
    println!("{}", seconds * 5);
    println!("{}", seconds * 10);
}