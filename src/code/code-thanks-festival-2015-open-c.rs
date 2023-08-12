// https://atcoder.jp/contests/code-thanks-festival-2015-open/tasks/code_thanks_festival_2015_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        H: [usize; N],
        X: usize,
    }
    for i in 0..N {
        if X < H[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", N + 1);
}