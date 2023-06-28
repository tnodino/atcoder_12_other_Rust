// https://atcoder.jp/contests/past202212-open/tasks/past202212_m

use proconio::input;
use proconio::fastout;
use std::cmp::{max, min};
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for i in 1..N {
        input! {
            U: usize,
            V: usize,
        }
        G[U-1].push((i, V - 1));
        G[V-1].push((i, U - 1));
    }
    let mut vec = vec![(N - 1, 1); N];
    let mut flg = vec![false; N];
    flg[0] = true;
    let mut que = VecDeque::new();
    que.push_back(0);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for (lb, nxt) in G[pos].iter() {
            if flg[*nxt] {
                continue;
            }
            vec[*nxt].0 = min(vec[pos].0, *lb);
            vec[*nxt].1 = max(vec[pos].1, *lb);
            flg[*nxt] = true;
            que.push_back(*nxt);
        }
    }
    let mut ans = (N - 1) * N / 2;
    for i in 1..N {
        ans += vec[i].0 * (N - vec[i].1);
    }
    println!("{}", ans);
}