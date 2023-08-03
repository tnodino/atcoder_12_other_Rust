// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_em

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
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    input! {
        Q: usize,
    }
    let mut flg = vec![true; M];
    let mut Query = vec![0; Q];
    let mut x = vec![0; Q];
    let mut u = vec![0; Q];
    let mut v = vec![0; Q];
    for i in 0..Q {
        input! {
            q: usize,
        }
        Query[i] = q;
        if q == 1 {
            input! {
                s: usize,
            }
            flg[s-1] = false;
            x[i] = s-1;
        }
        else {
            input! {
                s: usize,
                t: usize,
            }
            u[i] = s;
            v[i] = t;
        }
    }
    let mut UF = UnionFind::new(N+1);
    for i in 0..M {
        if flg[i] {
            UF.unite(A[i], B[i]);
        }
    }
    let mut ans = Vec::new();
    for i in (0..Q).rev() {
        if Query[i] == 1 {
            UF.unite(A[x[i]], B[x[i]]);
        }
        else {
            ans.push(match UF.same(u[i], v[i]) {
                true => "Yes",
                false => "No",
            });
        }
    }
    for i in (0..ans.len()).rev() {
        println!("{}", ans[i]);
    }
}