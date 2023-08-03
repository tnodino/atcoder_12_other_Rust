// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fl

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;

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
        }
        G[A-1].push((B-1, C));
        G[B-1].push((A-1, C));
    }
    let mut que = VecDeque::new();
    que.push_back(0);
    let mut dist1 = vec![INF; N];
    dist1[0] = 0;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (nxt, cost) in G[pos].iter() {
            if dist1[pos] + cost < dist1[*nxt] {
                dist1[*nxt] = dist1[pos] + cost;
                que.push_back(*nxt);
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back(N-1);
    let mut dist2 = vec![INF; N];
    dist2[N-1] = 0;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (nxt, cost) in G[pos].iter() {
            if dist2[pos] + cost < dist2[*nxt] {
                dist2[*nxt] = dist2[pos] + cost;
                que.push_back(*nxt);
            }
        }
    }
    let mut ans = 0;
    for i in 0..N {
        if dist1[i] + dist2[i] == dist1[N-1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}