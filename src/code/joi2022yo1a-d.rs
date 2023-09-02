// https://atcoder.jp/contests/joi2022yo1a/tasks/joi2022_yo1a_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..M {
            if A[i] == B[j] {
                ans += 1;
                break;
            }
        }
    }
    println!("{}", ans);
}