// https://atcoder.jp/contests/APG4b/tasks/APG4b_cf

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: usize,
        A: [usize; N],
        P: [usize; N],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            if A[i] + P[j] == S {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}