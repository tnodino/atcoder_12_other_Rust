// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_v

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
    let M = 100_000;
    let mut map = HashMap::new();
    let mut ans: usize = 0;
    for i in 0..N {
        let idx = M - A[i];
        if map.contains_key(&idx) {
            ans += map[&idx];
        }
        *map.entry(A[i]).or_insert(0) += 1;
    }
    println!("{}", ans);
}