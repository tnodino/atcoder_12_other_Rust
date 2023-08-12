// https://atcoder.jp/contests/joi2023yo1b/tasks/joi2023_yo1b_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        M: usize,
        B: [usize; M],
    }
    let mut ans = 0;
    for i in 0..N {
        ans += A[i];
        for j in 0..M {
            if ans == B[j] {
                ans = 0;
                break;
            }
        }
    }
    println!("{}", ans);
}