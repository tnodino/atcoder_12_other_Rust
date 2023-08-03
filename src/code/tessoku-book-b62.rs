// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ei

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn dfs(N: &usize, pos: usize, G: &Vec<Vec<usize>>, flg: &mut Vec<bool>, stc: &mut Vec<usize>) {
    if pos == N - 1 {
        println!("{}", stc.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        return;
    }
    for nxt in G[pos].iter() {
        if flg[*nxt] {
            continue;
        }
        stc.push(*nxt+1);
        flg[*nxt] = true;
        dfs(&N, *nxt, &G, flg, stc);
        stc.pop().unwrap();
    }
}

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
    let mut flg = vec![false; N];
    let mut stc = vec![1];
    flg[0] = true;
    dfs(&N, 0, &G, &mut flg, &mut stc);
}