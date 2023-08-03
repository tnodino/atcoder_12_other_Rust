// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_am

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>) {
    for nxt in G[pos].iter() {
        if flg[*nxt] {
            continue;
        }
        flg[*nxt] = true;
        dfs(*nxt, &G, flg);
    }
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
            A: usize,
            B: usize,
        }
        G[A-1].push(B-1);
        G[B-1].push(A-1);
    }
    let mut flg = vec![false; N];
    flg[0] = true;
    dfs(0, &G, &mut flg);
    if flg.iter().all(|&x| x) {
        println!("The graph is connected.");
    }
    else {
        println!("The graph is not connected.");
    }
}