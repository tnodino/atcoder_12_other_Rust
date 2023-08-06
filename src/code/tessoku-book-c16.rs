// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fn

use proconio::input;
use proconio::fastout;
use std::cmp::max;

const INF: isize = 1<<32;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: isize,
    }
    let mut A = vec![0; M+1];
    let mut B = vec![0; M+1];
    let mut vec = Vec::new();
    for i in 1..=M {
        input! {
            a: usize,
            S: isize,
            b: usize,
            T: isize,
        }
        A[i] = a;
        B[i] = b;
        vec.push((S, i, 2));
        vec.push((T + K, i, 1));
    }
    for i in 1..=N {
        vec.push((-1, i, 0));
        vec.push((INF, i, 0));
    }
    vec.sort_by(|a, b|
    if a.0 != b.0 {
        a.0.cmp(&b.0)
    }
    else {
        a.2.cmp(&b.2)
    });
    let K = vec.len();
    let mut idxs = vec![0; M+1];
    let mut idxt = vec![0; M+1];
    for i in 0..K {
        if vec[i].2 == 2 {
            idxs[vec[i].1] = i + 1;
        }
        if vec[i].2 == 1 {
            idxt[vec[i].1] = i + 1;
        }
    }
    let mut airport = vec![Vec::new(); N+1];
    for i in 0..K {
        if vec[i].2 == 0 {
            airport[vec[i].1].push(i+1);
        }
        if vec[i].2 == 1 {
            airport[B[vec[i].1]].push(i+1);
        }
        if vec[i].2 == 2 {
            airport[A[vec[i].1]].push(i+1);
        }
    }
    let mut G = vec![Vec::new(); K+2];
    for i in 1..=M {
        G[idxt[i]].push((idxs[i], 1));
    }
    for i in 1..=N {
        for j in 0..airport[i].len()-1 {
            let idx1 = airport[i][j];
            let idx2 = airport[i][j+1];
            G[idx2].push((idx1, 0));
        }
    }
    for i in 1..=N {
        G[airport[i][0]].push((0, 0));
        G[K+1].push((airport[i][airport[i].len()-1], 0));
    }
    let mut DP: Vec<usize> = vec![0; K+2];
    for i in 1..=K+1 {
        for j in 0..G[i].len() {
            DP[i] = max(DP[i], DP[G[i][j].0] + G[i][j].1);
        }
    }
    println!("{}", DP[K+1]);
}