// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_h

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    for _ in 0..N {
        input! {
            A: isize,
            B: isize,
        }
        if A == B {
            println!("-1");
        }
        else {
            println!("{}", (A - B).abs());
        }
    }
}