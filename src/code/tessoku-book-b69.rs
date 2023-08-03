// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ep

use proconio::input;
use proconio::fastout;

use std::cmp::min;
#[derive(Clone, Copy)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

#[allow(non_snake_case)]
struct FordFulkerson {
    sz: usize,
    used: Vec<bool>,
    G: Vec<Vec<Edge>>,
}

#[allow(non_snake_case)]
impl FordFulkerson {
    pub fn new(n: usize) -> Self {
        let sz = n;
        let used = vec![false; n];
        let G = vec![Vec::new(); n];
        Self {
            sz,
            used,
            G,
        }
    }

    pub fn add_edge(&mut self, a: usize, b: usize, c: usize) {
        let a_len = self.G[a].len();
        let b_len = self.G[b].len();
        self.G[a].push(Edge{to: b, cap: c, rev: b_len});
        self.G[b].push(Edge{to: a, cap: 0, rev: a_len});
    }

    fn dfs(&mut self, pos: usize, goal: usize, F: usize) -> usize {
        if pos == goal {
            return F
        }
        self.used[pos] = true;
        for i in 0..self.G[pos].len() {
            if self.G[pos][i].cap == 0 {
                continue;
            }
            if self.used[self.G[pos][i].to] {
                continue;
            }
            let to = self.G[pos][i].to;
            let rev = self.G[pos][i].rev;
            let flow = self.dfs(self.G[pos][i].to, goal, min(F, self.G[pos][i].cap));
            if flow >= 1 {
                self.G[pos][i].cap -= flow;
                self.G[to][rev].cap += flow;
                return flow;
            }
        }
        return 0;
    }

    pub fn max_flow(&mut self, s: usize, t: usize) -> usize {
        let mut ret = 0;
        loop {
            for i in 0..self.sz {
                self.used[i] = false;
            }
            let F = self.dfs(s, t, 1_000_000_000);
            if F == 0 {
                break;
            }
            ret += F;
        }
        return ret;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut C = Vec::new();
    for _ in 0..N {
        input! {
            c: String,
        }
        let c = c.chars().collect::<Vec<char>>();
        C.push(c);
    }
    let K = N + 24;
    let mut FF = FordFulkerson::new(K+2);
    for i in 1..=N {
        FF.add_edge(0, i, 10);
    }
    for i in N+1..=K {
        FF.add_edge(i, K+1, M);
    }
    for i in 0..N {
        for j in 0..24 {
            if C[i][j] == '1' {
                FF.add_edge(i+1, N+1+j, 1);
            }
        }
    }
    if FF.max_flow(0, K+1) == M * 24 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}