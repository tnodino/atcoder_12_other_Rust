// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_f

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ma = 0;
    let mut ans = 0;
    for i in 0..N {
        if ma < a[i] {
            ans += 1;
            ma = a[i];
        }
    }
    println!("{}", ans);
}