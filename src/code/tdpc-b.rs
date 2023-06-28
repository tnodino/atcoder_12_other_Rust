// https://atcoder.jp/contests/tdpc/tasks/tdpc_game

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};

const INF: usize = 1<<32;

#[allow(non_snake_case)]
fn f(flg: bool, l: usize, r: usize, a: &Vec<usize>, b: &Vec<usize>, DP: &mut Vec<Vec<usize>>) -> usize {
    if DP[l][r] != INF {
        return DP[l][r];
    }
    let mut ret;
    if flg {
        ret = 0;
        if l < a.len() {
            ret = max(ret, f(flg ^ true, l+1, r, a, b, DP) + a[l]);
        }
        if r < b.len() {
            ret = max(ret, f(flg ^ true, l, r+1, a, b, DP) + b[r]);
        }
    }
    else {
        ret = INF;
        if l < a.len() {
            ret = min(ret, f(flg ^ true, l+1, r, a, b, DP));
        }
        if r < b.len() {
            ret = min(ret, f(flg ^ true, l, r+1, a, b, DP));
        }
    }
    DP[l][r] = ret;
    return DP[l][r]
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        a: [usize; A],
        b: [usize; B],
    }
    let mut DP = vec![vec![INF; B+1]; A+1];
    DP[A][B] = 0;
    f(true, 0, 0, &a, &b, &mut DP);
    println!("{}", DP[0][0]);
}