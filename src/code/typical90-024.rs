// https://atcoder.jp/contests/typical90/tasks/typical90_x

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
        A: [isize; N],
        B: [isize; N],
    }
    let mut cnt = 0;
    for i in 0..N {
        cnt += (A[i] - B[i]).abs();
    }
    if cnt > K || cnt % 2 != K % 2 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}