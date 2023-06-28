// https://atcoder.jp/contests/tdpc/tasks/tdpc_dice

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[allow(non_snake_case)]
fn check(mut D: usize) -> bool {
    while D % 2 == 0 {
        D /= 2;
    }
    while D % 3 == 0 {
        D /= 3;
    }
    while D % 5 == 0 {
        D /= 5;
    }
    if D > 1 {
        return true;
    }
    return false;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        D: usize,
    }
    if check(D) {
        println!("0");
        return;
    }
    let p = 1. / 6.;
    let mut DP = vec![HashMap::new(); N+1];
    DP[0].insert(1, 1.);
    for i in 0..N {
        DP[i+1].insert(0, 0.);
        for (k, v) in DP[i].clone() {
            for d in 1..=6 {
                let x = (k * d) % D;
                *DP[i+1].entry(x).or_insert(0.) += v * p;
            }
        }
    }
    println!("{}", DP[N].get(&0).unwrap());
}