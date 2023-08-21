// https://atcoder.jp/contests/typical90/tasks/typical90_n

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
        mut B: [isize; N],
    }
    A.sort();
    B.sort();
    let mut ans = 0;
    for i in 0..N {
        ans += (A[i] - B[i]).abs();
    }
    println!("{}", ans);
}