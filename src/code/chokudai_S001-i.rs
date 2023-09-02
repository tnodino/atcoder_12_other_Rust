// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_i

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut l = 0;
    let mut s = 0;
    let mut ans = 0;
    for r in 0..N {
        s += a[r];
        while l <= r && s > N {
            s -= a[l];
            l += 1;
        }
        if s == N {
            ans += 1;
        }
    }
    println!("{}", ans);
}