// https://atcoder.jp/contests/APG4b/tasks/APG4b_bz

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut map = HashMap::new();
    for i in 0..N {
        *map.entry(A[i]).or_insert(0) += 1;
    }
    let ma = *map.values().max().unwrap();
    for (k, v) in map {
        if v == ma {
            println!("{} {}", k, v);
            return;
        }
    }
}