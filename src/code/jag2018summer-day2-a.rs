// https://atcoder.jp/contests/jag2018summer-day2/tasks/jag2018summer_day2_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    }
    let M = 1_000_000_007;
    let mut ans = z;
    while ans % 17 != x || ans % 107 != y {
        ans += M;
    }
    println!("{}", ans);
}