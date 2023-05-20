// https://atcoder.jp/contests/agc001/tasks/agc001_c

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;
use std::cmp::min;

#[allow(non_snake_case)]
fn bfs(sx1: usize, sx2: usize, N: usize, M: usize, G: &Vec<Vec<usize>>) -> usize {
    let mut dist = vec![N; N];
    dist[sx1] = 0;
    dist[sx2] = 0;
    let mut que = VecDeque::new();
    que.push_back(sx1);
    que.push_back(sx2);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if dist[*nxt] == N {
                dist[*nxt] = dist[pos] + 1;
                que.push_back(*nxt);
            }
        }
    }
    let mut cnt = 0;
    for i in 0..N {
        if dist[i] > M {
            cnt += 1;
        }
    }
    return cnt;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
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
    let M = K / 2;
    let mut ans = N;
    if K % 2 == 0 {
        for i in 0..N {
            ans = min(ans, bfs(i, i, N, M, &G));
        }
    }
    else {
        for i in 0..N {
            for j in G[i].iter() {
                ans = min(ans, bfs(i, *j, N, M, &G));
            }
        }
    }
    println!("{}", ans);
}