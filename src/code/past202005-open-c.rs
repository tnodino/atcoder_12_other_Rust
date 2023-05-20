// https://atcoder.jp/contests/past202005-open/tasks/past202005_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut A: usize,
        R: usize,
        N: usize,
    }
    let M = 1_000_000_000;
    if A == 1 && R == 1 {
        println!("1");
        return;
    }
    for _ in 0..N-1 {
        A *= R;
        if A > M {
            println!("large");
            return;
        }
    }
    println!("{}", A);
}