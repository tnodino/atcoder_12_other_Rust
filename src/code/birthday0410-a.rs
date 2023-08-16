// https://atcoder.jp/contests/birthday0410/tasks/birthday0410_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _t: usize,
    }
    let mi1 = -2147483648;
    let mi2 = -2147483647;
    println!("{} {}", mi2, -mi2);
    println!("{} {}", mi1, 0);
    println!("{} {}", -1, mi2);
    println!("0 0");
}