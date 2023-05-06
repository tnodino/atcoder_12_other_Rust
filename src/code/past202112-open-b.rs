// https://atcoder.jp/contests/past202112-open/tasks/past202112_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        let mut x = B - A;
        x %= 100;
        ans += x / 50;
        x %= 10;
        ans += x / 5;
    }
    println!("{}", ans);
}