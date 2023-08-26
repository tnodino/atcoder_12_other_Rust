// https://atcoder.jp/contests/joi2021yo1c/tasks/joi2021_yo1c_c

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
            if A[i] <= B[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}