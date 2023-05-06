// https://atcoder.jp/contests/joi2020yo1a/tasks/joi2020_yo1a_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        S: String,
    }
    let mut ans = 0;
    for s in S.chars() {
        ans += match s {
            'a' => 1,
            'i' => 1,
            'u' => 1,
            'e' => 1,
            'o' => 1,
            _ => 0,
        }
    }
    println!("{}", ans);
}