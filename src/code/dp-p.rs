// https://atcoder.jp/contests/dp/tasks/dp_p

use proconio::input;
use proconio::fastout;

const MOD: usize = 1_000_000_007;

#[allow(non_snake_case)]
fn dfs(pos: usize, rev: usize, DP: &mut Vec<Vec<usize>>, G: &Vec<Vec<usize>>) {
    for nxt in G[pos].iter() {
        if *nxt == rev {
            continue;
        }
        dfs(*nxt, pos, DP, &G);
        DP[pos][0] *= DP[*nxt][0] + DP[*nxt][1];
        DP[pos][1] *= DP[*nxt][0];
        DP[pos][0] %= MOD;
        DP[pos][1] %= MOD;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            x: usize,
            y: usize,
        }
        G[x-1].push(y-1);
        G[y-1].push(x-1);
    }
    let mut DP = vec![vec![1; 2]; N];
    dfs(0, N, &mut DP, &G);
    println!("{}", (DP[0][0] + DP[0][1]) % MOD);
}