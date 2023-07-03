// https://atcoder.jp/contests/abc306/tasks/abc306_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [usize; 64],
    }
    let mut ans = 0;
    for i in 0..64 {
        ans += A[i] * (1 << i);
    }
    println!("{}", ans);
}