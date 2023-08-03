// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_el

use proconio::input;
use proconio::fastout;
use std::cmp::max;

#[allow(non_snake_case)]
fn dfs(pos: usize, pre: usize, G: &Vec<Vec<usize>>, DP: &mut Vec<usize>) -> usize {
    for nxt in G[pos].iter() {
        if *nxt == pre {
            continue;
        }
        DP[pos] = max(DP[pos], dfs(*nxt, pos, &G, DP));
    }
    return DP[pos] + 1;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            A: usize,
            B: usize,
        }
        G[A-1].push(B-1);
        G[B-1].push(A-1);
    }
    let mut DP = vec![0; N];
    dfs(T-1, N, &G, &mut DP);
    println!("{}", DP.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}