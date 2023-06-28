// https://atcoder.jp/contests/past202104-open/tasks/past202104_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [isize; N],
    }
    let mut s = vec![0; N+1];
    for i in 0..N {
        s[i+1] += s[i] + A[i];
    }
    for i in K..=N {
        println!("{}", s[i] - s[i-K]);
    }
}