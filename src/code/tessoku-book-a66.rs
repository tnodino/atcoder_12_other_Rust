// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bn

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
        Q: usize,
    }
    let mut UF = UnionFind::new(N+1);
    for _ in 0..Q {
        input! {
            q: usize,
            u: usize,
            v: usize,
        }
        if q == 1 {
            UF.unite(u, v);
        }
        else {
            println!("{}", match UF.same(u, v) {
                true => "Yes",
                false => "No",
            });
        }
    }
}