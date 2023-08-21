// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    for i in 0..N {
        if a[i] == 1 {
            println!("{}", i + 1);
            return;
        }
    }
}