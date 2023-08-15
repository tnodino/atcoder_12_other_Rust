// https://atcoder.jp/contests/gigacode-2019/tasks/gigacode_2019_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        Z: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        if A >= X && B >= Y && A + B >= Z {
            ans += 1;
        }
    }
    println!("{}", ans);
}