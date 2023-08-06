// https://atcoder.jp/contests/joi2006yo/tasks/joi2006yo_b

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            a: char,
            b: char,
        }
        map.insert(a, b);
    }
    input! {
        m: usize,
    }
    for _ in 0..m {
        input! {
            c: char,
        }
        if map.contains_key(&c) {
            print!("{}", map[&c]);
        }
        else {
            print!("{}", c);
        }
    }
    println!();
}