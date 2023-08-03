// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bu

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 10_000;

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
            C: usize,
            D: usize,
        }
        G[A-1].push((B-1, C*INF-D));
        G[B-1].push((A-1, C*INF-D));
    }
    let mut dist = vec![INF*INF; N];
    dist[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (nxt, cost) in G[pos].iter() {
            if dist[pos] + *cost < dist[*nxt] {
                dist[*nxt] = dist[pos] + *cost;
                que.push_back(*nxt);
            }
        }
    }
    println!("{} {}", (dist[N-1] + INF - 1) / INF, (INF - (dist[N-1] % INF)) % INF);
}