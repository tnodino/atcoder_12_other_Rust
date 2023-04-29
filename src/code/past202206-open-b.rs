// https://atcoder.jp/contests/past202206-open/tasks/past202206_b

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut map = HashMap::new();
    for i in 0..S.len()-1 {
        *map.entry(&S[i..i+2]).or_insert(0) += 1;
    }
    let ma = *map.values().max().unwrap();
    let mut ans = "zz";
    for (k, v) in map {
        if v == ma && k < ans {
            ans = k;
        }
    }
    println!("{}", ans);
}