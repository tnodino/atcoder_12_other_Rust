// https://atcoder.jp/contests/typical90/tasks/typical90_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
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
    let mut st = 0;
    let mut ans = 0;
    for _ in 0..2 {
        let mut que = VecDeque::new();
        que.push_back(st);
        let mut dist = vec![-1; N];
        dist[st] = 1;
        while !que.is_empty() {
            let pos = que.pop_front().unwrap();
            for nxt in G[pos].iter() {
                if dist[*nxt] == -1 {
                    dist[*nxt] = dist[pos] + 1;
                    que.push_back(*nxt);
                }
            }
        }
        for i in 0..N {
            if ans < dist[i] {
                st = i;
                ans = dist[i];
            }
        }
    }
    println!("{}", ans);
}