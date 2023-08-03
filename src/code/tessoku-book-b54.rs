// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ea

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..N {
        input! {
            A: usize,
        }
        *map.entry(A).or_insert(0 as usize) += 1;
    }
    let mut ans = 0;
    for (_, v) in map.iter() {
        ans += (v - 1) * v / 2;
    }
    println!("{}", ans);
}