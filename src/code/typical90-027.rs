// https://atcoder.jp/contests/typical90/tasks/typical90_aa

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    for i in 1..=N {
        input! {
            S: String,
        }
        if set.contains(&S) {
            continue;
        }
        println!("{}", i);
        set.insert(S);
    }
}