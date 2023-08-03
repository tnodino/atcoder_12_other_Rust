// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_an

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

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
    let mut dist = vec![-1; N];
    dist[0] = 0;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if dist[*nxt] != -1 {
                continue;
            }
            dist[*nxt] = dist[pos] + 1;
            que.push_back(*nxt);
        }
    }
    for i in 0..N {
        println!("{}", dist[i]);
    }
}