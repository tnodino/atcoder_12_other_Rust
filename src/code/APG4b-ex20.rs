// https://atcoder.jp/contests/APG4b/tasks/APG4b_cc

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        p: [usize; N-1],
    }
    let mut ans = vec![1; N];
    for i in (0..N-1).rev() {
        ans[p[i]] += ans[i+1];
    }
    for a in ans {
        println!("{}", a);
    }
}