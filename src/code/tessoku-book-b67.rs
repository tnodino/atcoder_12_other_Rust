// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_en

use proconio::input;
use proconio::fastout;

struct UnionFind {
    par: Vec<isize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = vec![-1; n];
        let sz = vec![1; n];
        Self {
            par,
            sz,
        }
    }

    pub fn root(&mut self, mut x: usize) -> usize {
        while self.par[x] != -1 {
            x = self.par[x] as usize;
        }
        return x;
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let u = self.root(u);
        let v = self.root(v);
        if u == v {
            return;
        }
        if self.sz[u] < self.sz[v] {
            self.par[u] = v as isize;
            self.sz[v] += self.sz[u];
        }
        else {
            self.par[v] = u as isize;
            self.sz[u] += self.sz[v];
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        if self.root(u) == self.root(v) {
            return true;
        }
        return false;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
            C: usize,
        }
        vec.push((C, A, B));
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    let mut UF = UnionFind::new(N+1);
    let mut ans = 0;
    for i in 0..M {
        if !UF.same(vec[i].1, vec[i].2) {
            UF.unite(vec[i].1, vec[i].2);
            ans += vec[i].0;
        }
    }
    println!("{}", ans);
}