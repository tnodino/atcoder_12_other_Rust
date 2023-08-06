// https://atcoder.jp/contests/joi2006ho/tasks/joi2006ho_d

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[allow(non_snake_case)]

fn dfs(pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>) -> usize {
    let mut ret = flg.iter().filter(|&x| *x).count();
    for nxt in G[pos].iter() {
        if flg[*nxt] {
            continue;
        }
        flg[*nxt] = true;
        ret = max(ret, dfs(*nxt, &G, flg));
        flg[*nxt] = false;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let m = 100;
    let mut G = vec![Vec::new(); m+1];
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
        }
        G[a].push(b);
        G[b].push(a);
    }
    let mut flg = vec![false; m+1];
    let mut ans = 0;
    for i in 1..=m {
        flg[i] = true;
        ans = max(ans, dfs(i, &G, &mut flg));
        flg[i] = false;
    }
    println!("{}", ans);
}