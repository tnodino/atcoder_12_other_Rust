// https://atcoder.jp/contests/abc209/tasks/abc209_d

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            a: usize,
            b: usize,
        }
        G[a-1].push(b-1);
        G[b-1].push(a-1);
    }
    let mut flg = vec![0; N];
    let mut que = VecDeque::new();
    que.push_back((0, N));
    while !que.is_empty() {
        let (pos, rev) = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if rev == *nxt {
                continue;
            }
            flg[*nxt] = flg[pos] ^ 1;
            que.push_back((*nxt, pos));
        }
    }
    for _ in 0..Q {
        input! {
            c: usize,
            d: usize,
        }
        if flg[c-1] == flg[d-1] {
            println!("Town");
        }
        else {
            println!("Road");
        }
    }
}