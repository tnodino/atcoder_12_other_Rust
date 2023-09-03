// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_f

use proconio::input;
use proconio::fastout;
use std::mem::swap;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..N {
        input! {
            mut A: usize,
            mut B: usize,
        }
        if A > B {
            swap(&mut A, &mut B);
        }
        set.insert((A, B));
    }
    println!("{}", set.len());
}