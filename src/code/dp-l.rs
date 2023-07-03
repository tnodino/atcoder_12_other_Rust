// https://atcoder.jp/contests/dp/tasks/dp_l

use proconio::input;
use proconio::fastout;
use std::cmp::{min, max};

#[allow(non_snake_case)]
fn f(flg: bool, l: usize, r: usize, a: &Vec<isize>, DP: &mut Vec<Vec<isize>>) -> isize {
    if DP[l][r] != 0 {
        return DP[l][r];
    }
    if flg {
        let x;
        if l == r {
            x = a[l];
        }
        else {
            x = max(f(false, l+1, r, &a, DP) + a[l], f(false, l, r-1, &a, DP) + a[r]);
        }
        DP[l][r] = x;
    }
    else {
        let x;
        if l == r {
            x = -a[l];
        }
        else {
            x = min(f(true, l+1, r, &a, DP) - a[l], f(true, l, r-1, &a, DP) - a[r]);
        }
        DP[l][r] = x;
    }
    return DP[l][r];
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [isize; N],
    }
    let mut DP = vec![vec![0; N]; N];
    println!("{}", f(true, 0, N-1, &a, &mut DP));
}