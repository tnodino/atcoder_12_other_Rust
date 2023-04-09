// https://atcoder.jp/contests/agc001/tasks/agc001_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut L: [usize; N*2],
    }
    L.sort();
    let mut ans = 0;
    for i in (0..N*2).step_by(2) {
        ans += L[i];
    }
    println!("{}", ans);
}