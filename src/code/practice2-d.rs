// https://atcoder.jp/contests/practice2/tasks/practice2_d

use proconio::input;
use proconio::fastout;

use std::cmp::min;
use std::iter;

#[derive(Default)]
pub struct SimpleQueue<T> {
    payload: Vec<T>,
    pos: usize,
}

impl<T> SimpleQueue<T> {
    pub fn reserve(&mut self, n: usize) {
        if n > self.payload.len() {
            self.payload.reserve(n - self.payload.len());
        }
    }

    pub fn size(&self) -> usize {
        return self.payload.len() - self.pos;
    }

    pub fn empty(&self) -> bool {
        return self.pos == self.payload.len();
    }

    pub fn push(&mut self, t: T) {
        self.payload.push(t);
    }

    pub fn front(&self) -> Option<&T> {
        if self.pos < self.payload.len() {
            return Some(&self.payload[self.pos]);
        } else {
            return None;
        }
    }

    pub fn clear(&mut self) {
        self.payload.clear();
        self.pos = 0;
    }

    pub fn pop(&mut self) -> Option<&T> {
        if self.pos < self.payload.len() {
            self.pos += 1;
            return Some(&self.payload[self.pos - 1]);
        } else {
            return None;
        }
    }
}

use std::{
    fmt,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
};

pub trait Integral:
    'static
    + Send
    + Sync
    + Copy
    + Ord
    + Not<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + Sum
    + Product
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + BitXor<Output = Self>
    + BitOrAssign
    + BitAndAssign
    + BitXorAssign
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + fmt::Display
    + fmt::Debug
    + fmt::Binary
    + fmt::Octal
    + Zero
    + One
    + BoundedBelow
    + BoundedAbove
{
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }

            impl Integral for $ty {}
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl<Cap> MfGraph<Cap>
where
    Cap: Integral,
{
    pub fn new(n: usize) -> MfGraph<Cap> {
        MfGraph {
            _n: n,
            pos: Vec::new(),
            g: iter::repeat_with(Vec::new).take(n).collect(),
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: Cap) -> usize {
        assert!(from < self._n);
        assert!(to < self._n);
        assert!(Cap::zero() <= cap);
        let m = self.pos.len();
        self.pos.push((from, self.g[from].len()));
        let rev = self.g[to].len() + usize::from(from == to);
        self.g[from].push(_Edge { to, rev, cap });
        let rev = self.g[from].len() - 1;
        self.g[to].push(_Edge {
            to: from,
            rev,
            cap: Cap::zero(),
        });
        return m;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Edge<Cap: Integral> {
    pub from: usize,
    pub to: usize,
    pub cap: Cap,
    pub flow: Cap,
}

impl<Cap> MfGraph<Cap>
where
    Cap: Integral,
{
    pub fn get_edge(&self, i: usize) -> Edge<Cap> {
        let m = self.pos.len();
        assert!(i < m);
        let _e = &self.g[self.pos[i].0][self.pos[i].1];
        let _re = &self.g[_e.to][_e.rev];
        Edge {
            from: self.pos[i].0,
            to: _e.to,
            cap: _e.cap + _re.cap,
            flow: _re.cap,
        }
    }

    pub fn edges(&self) -> Vec<Edge<Cap>> {
        let m = self.pos.len();
        return (0..m).map(|i| self.get_edge(i)).collect();
    }

    pub fn change_edge(&mut self, i: usize, new_cap: Cap, new_flow: Cap) {
        let m = self.pos.len();
        assert!(i < m);
        assert!(Cap::zero() <= new_flow && new_flow <= new_cap);
        let (to, rev) = {
            let _e = &mut self.g[self.pos[i].0][self.pos[i].1];
            _e.cap = new_cap - new_flow;
            (_e.to, _e.rev)
        };
        let _re = &mut self.g[to][rev];
        _re.cap = new_flow;
    }

    pub fn flow(&mut self, s: usize, t: usize) -> Cap {
        return self.flow_with_capacity(s, t, Cap::max_value());
    }

    pub fn flow_with_capacity(&mut self, s: usize, t: usize, flow_limit: Cap) -> Cap {
        let n_ = self._n;
        let mut calc = FlowCalculator {
            graph: self,
            s,
            t,
            flow_limit,
            level: vec![0; n_],
            iter: vec![0; n_],
            que: SimpleQueue::default(),
        };
        let mut flow = Cap::zero();
        while flow < flow_limit {
            calc.bfs();
            if calc.level[t] == -1 {
                break;
            }
            calc.iter.iter_mut().for_each(|e| *e = 0);
            while flow < flow_limit {
                let f = calc.dfs(t, flow_limit - flow);
                if f == Cap::zero() {
                    break;
                }
                flow += f;
            }
        }
        return flow;
    }

    pub fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self._n];
        let mut que = SimpleQueue::default();
        que.push(s);
        while let Some(&p) = que.pop() {
            visited[p] = true;
            for e in &self.g[p] {
                if e.cap != Cap::zero() && !visited[e.to] {
                    visited[e.to] = true;
                    que.push(e.to);
                }
            }
        }
        return visited;
    }
}

#[allow(dead_code)]
struct FlowCalculator<'a, Cap> {
    graph: &'a mut MfGraph<Cap>,
    s: usize,
    t: usize,
    flow_limit: Cap,
    level: Vec<i32>,
    iter: Vec<usize>,
    que: SimpleQueue<usize>,
}

impl<Cap> FlowCalculator<'_, Cap>
where
    Cap: Integral,
{
    fn bfs(&mut self) {
        self.level.iter_mut().for_each(|e| *e = -1);
        self.level[self.s] = 0;
        self.que.clear();
        self.que.push(self.s);
        while !self.que.empty() {
            let v = *self.que.front().unwrap();
            self.que.pop();
            for e in &self.graph.g[v] {
                if e.cap == Cap::zero() || self.level[e.to] >= 0 {
                    continue;
                }
                self.level[e.to] = self.level[v] + 1;
                if e.to == self.t {
                    return;
                }
                self.que.push(e.to);
            }
        }
    }
    fn dfs(&mut self, v: usize, up: Cap) -> Cap {
        if v == self.s {
            return up;
        }
        let mut res = Cap::zero();
        let level_v = self.level[v];
        for i in self.iter[v]..self.graph.g[v].len() {
            self.iter[v] = i;
            let &_Edge {
                to: e_to,
                rev: e_rev,
                ..
            } = &self.graph.g[v][i];
            if level_v <= self.level[e_to] || self.graph.g[e_to][e_rev].cap == Cap::zero() {
                continue;
            }
            let d = self.dfs(e_to, min(up - res, self.graph.g[e_to][e_rev].cap));
            if d <= Cap::zero() {
                continue;
            }
            self.graph.g[v][i].cap += d;
            self.graph.g[e_to][e_rev].cap -= d;
            res += d;
            if res == up {
                return res;
            }
        }
        self.iter[v] = self.graph.g[v].len();
        return res
    }
}

#[derive(Default)]
pub struct MfGraph<Cap> {
    _n: usize,
    pos: Vec<(usize, usize)>,
    g: Vec<Vec<_Edge<Cap>>>,
}

struct _Edge<Cap> {
    to: usize,
    rev: usize,
    cap: Cap,
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    let mut MG = MfGraph::new(N * M + 2);
    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 0 {
                MG.add_edge(N * M, i * M + j, 1);
            }
            else {
                MG.add_edge(i * M + j, N * M + 1, 1);
            }
        }
    }
    for i in 0..N {
        for j in 0..M {
            if (i + j) % 2 == 1 {
                continue;
            }
            if S[i][j] == '#' {
                continue;
            }
            if i + 1 < N && S[i+1][j] == '.' {
                MG.add_edge(i * M + j, (i + 1) * M + j, 1);
            }
            if 0 < i && S[i-1][j] == '.' {
                MG.add_edge(i * M + j, (i - 1) * M + j, 1);
            }
            if j + 1 < M && S[i][j+1] == '.' {
                MG.add_edge(i * M + j, i * M + (j + 1), 1);
            }
            if 0 < j && S[i][j-1] == '.' {
                MG.add_edge(i * M + j, i * M + (j - 1), 1);
            }
        }
    }
    println!("{}", MG.flow(N * M, N * M + 1));
    for e in MG.edges() {
        if e.from == N * M || e.to == N * M + 1 || e.flow  == 0 {
            continue;
        }
        let from_x = e.from / M;
        let from_y = e.from % M;
        let to_x = e.to / M;
        let to_y = e.to % M;
        if to_x + 1 < N && from_x == to_x + 1 {
            S[from_x][from_y] = '^';
            S[to_x][to_y] = 'v';
        }
        if 0 < to_x && from_x == to_x - 1 {
            S[from_x][from_y] = 'v';
            S[to_x][to_y] = '^';
        }
        if to_y + 1 < M && from_y == to_y + 1 {
            S[from_x][from_y] = '<';
            S[to_x][to_y] = '>';
        }
        if 0 < to_y && from_y == to_y - 1 {
            S[from_x][from_y] = '>';
            S[to_x][to_y] = '<';
        }
    }
    for i in 0..N {
        println!("{}", S[i].iter().collect::<String>());
    }
}