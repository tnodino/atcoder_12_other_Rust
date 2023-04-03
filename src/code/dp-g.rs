// https://atcoder.jp/contests/dp/tasks/dp_g

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &Vec<Vec<usize>>, DP: &mut Vec<isize>) {
    let mut ret = 0;
    for nxt in G[pos].iter() {
        if DP[*nxt] == -1 {
            dfs(*nxt, &G, DP);
        }
        ret = max(ret, DP[*nxt] + 1);
    }
    DP[pos] = ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..M {
        input! {
            x: usize,
            y: usize,
        }
        G[x-1].push(y-1);
    }
    let mut DP = vec![-1; N];
    for i in 0..N {
        if DP[i] == -1 {
            dfs(i, &G, &mut DP);
        }
    }
    println!("{}", DP.iter().max().unwrap());
}