// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bl

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

const INF: isize = 1_000_000_000;

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
            C: isize,
        }
        G[A-1].push((B-1, -C));
        G[B-1].push((A-1, -C));
    }
    let mut bh = BinaryHeap::new();
    bh.push((0, 0));
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut flg = vec![false; N];
    while !bh.is_empty() {
        let (p_cost, pos) = bh.pop().unwrap();
        if flg[pos] {
            continue;
        }
        flg[pos] = true;
        for (nxt, n_cost) in G[pos].iter() {
            if -(p_cost + *n_cost) < dist[*nxt] {
                dist[*nxt] = -(p_cost + *n_cost);
                bh.push((-dist[*nxt], *nxt));
            }
        }
    }
    for i in 0..N {
        if dist[i] == INF {
            dist[i] = -1;
        }
        println!("{}", dist[i]);
    }
}