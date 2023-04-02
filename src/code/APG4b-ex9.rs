// https://atcoder.jp/contests/APG4b/tasks/APG4b_cn

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: isize,
        a: isize,
        b: isize,
    }
    let mut ans = x;
    ans += 1;
    println!("{}", ans);
    ans *= a + b;
    println!("{}", ans);
    ans *= ans;
    println!("{}", ans);
    ans -= 1;
    println!("{}", ans);
}