// https://atcoder.jp/contests/past202004-open/tasks/past202004_d

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut set = HashSet::new();
    for n in 1..=3 {
        if N < n {
            break;
        }
        for i in 0..(N-n+1) {
            for bit in 0..1<<n {
                let mut vec = Vec::new();
                for k in 0..n {
                    if bit & (1 << k) > 0 {
                        vec.push('.');
                    }
                    else {
                        vec.push(S[i+k]);
                    }
                }
                set.insert(vec.iter().collect::<String>());
            }
        }
    }
    println!("{}", set.len());
}