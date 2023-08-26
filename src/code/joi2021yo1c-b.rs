// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_b

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
    for (i, s) in S.chars().enumerate() {
        if i % 2 == 0 && s != 'I' {
            ans += 1;
        }
        if i % 2 == 1 && s != 'O' {
            ans += 1;
        }
    }
    println!("{}", ans);
}