// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ek

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

const INF: isize = 1<<30;

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
        G[A-1].push((B-1, C));
        G[B-1].push((A-1, C));
    }
    let mut dist = vec![INF; N];
    dist[0] = 0;
    let mut bh = BinaryHeap::new();
    bh.push((0, 0));
    while !bh.is_empty() {
        let (_, pos) = bh.pop().unwrap();
        for (nxt, cost) in G[pos].iter() {
            if dist[pos] + cost < dist[*nxt] {
                dist[*nxt] = dist[pos] + cost;
                bh.push((-(dist[*nxt]), *nxt));
            }
        }
    }
    let mut ans = vec![N];
    let mut pos = N - 1;
    while pos != 0 {
        for (nxt, cost) in G[pos].iter() {
            if dist[*nxt] + cost == dist[pos] {
                ans.push(*nxt+1);
                pos = *nxt;
                break;
            }
        }
    }
    println!("{}", ans.iter().rev().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}