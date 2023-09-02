// https://atcoder.jp/contests/joi2023ho/tasks/joi2023ho_a

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
        *map.entry(A[i]).or_insert(0) = i;
    }
    let mut ans = vec![0; N];
    let mut idx = 0;
    while idx < N {
        for i in idx..=map[&A[idx]] {
            ans[i] = A[idx];
        }
        idx = map[&A[idx]] + 1;
    }
    for i in 0..N {
        println!("{}", ans[i]);
    }
}