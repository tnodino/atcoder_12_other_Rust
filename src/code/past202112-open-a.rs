// https://atcoder.jp/contests/past202112-open/tasks/past202112_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        h: usize,
        w: usize,
    }
    if H <= h && w <= W {
        println!("Yes");
    }
    else {
        println!("No");
    }
}