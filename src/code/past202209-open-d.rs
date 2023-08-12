// https://atcoder.jp/contests/past202209-open/tasks/past202209_d

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;
use std::mem::swap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..M {
        input! {
            mut u: usize,
            mut v: usize,
        }
        if N < u || N < v {
            println!("No");
            return;
        }
        if u == v {
            println!("No");
            return;
        }
        if u > v {
            swap(&mut u, &mut v);
        }
        if set.contains(&(u, v)) {
            println!("No");
            return;
        }
        set.insert((u, v));
    }
    println!("Yes");
}