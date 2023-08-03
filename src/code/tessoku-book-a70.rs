// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_br

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
        A: [usize; N],
    }
    let mut edge = Vec::new();
    for _ in 0..M {
        input! {
            X: usize,
            Y: usize,
            Z: usize,
        }
        let idx = (1<<(X-1)) + (1<<(Y-1)) + (1<<(Z-1));
        edge.push(idx);
    }
    let B = 1 << N;
    let mut idx = 0;
    for i in 0..N {
        idx += (1 << i) * A[i];
    }
    let mut que = VecDeque::new();
    que.push_back(idx);
    let mut dist = vec![INF; B];
    dist[idx] = 0;
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for i in 0..M {
            if dist[pos] + 1 < dist[pos^edge[i]] {
                dist[pos^edge[i]] = dist[pos] + 1;
                que.push_back(pos^edge[i]);
            }
        }
    }
    if dist[B-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dist[B-1]);
    }
}