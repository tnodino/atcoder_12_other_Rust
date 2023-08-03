// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fk

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret = (ret * x) % MOD;
        }
        x = (x * x) % MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        P: usize,
        mut A: [usize; N],
    }
    for i in 0..N {
        A[i] %= MOD;
    }
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut ans: usize = 0;
    for i in 0..N {
        if A[i] == 0 {
            if P == 0 {
                ans += i;
            }
        }
        else {
            let x = P * bin_power(A[i], MOD-2) % MOD;
            if map.contains_key(&x) {
                ans += map[&x];
            }
        }
        *map.entry(A[i]).or_insert(0) += 1;
    }
    println!("{}", ans);
}