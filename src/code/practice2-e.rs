// https://atcoder.jp/contests/practice2/tasks/practice2_e

use proconio::input;
use proconio::fastout;

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

pub struct MinCostFlowEdge<T> {
    pub from: usize,
    pub to: usize,
    pub cap: T,
    pub flow: T,
    pub cost: T,
}

pub struct MinCostFlowGraph<T> {
    pos: Vec<(usize, usize)>,
    g: Vec<Vec<_Edge<T>>>,
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
impl<T> MinCostFlowGraph<T>
where
    T: Integral + std::ops::Neg<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            pos: vec![],
            g: (0..n).map(|_| vec![]).collect(),
        }
    }

    pub fn get_edge(&self, i: usize) -> MinCostFlowEdge<T> {
        let e = &self.g[self.pos[i].0][self.pos[i].1];
        let re = &self.g[e.to][e.rev];
        MinCostFlowEdge {
            from: self.pos[i].0,
            to: e.to,
            cap: e.cap + re.cap,
            flow: re.cap,
            cost: e.cost,
        }
    }

    pub fn edges(&self) -> Vec<MinCostFlowEdge<T>> {
        let m = self.pos.len();
        let mut result = vec![];
        for i in 0..m {
            result.push(self.get_edge(i));
        }
        return result;
    }

    pub fn add_edge(&mut self, from: usize, to: usize, cap: T, cost: T) -> usize {
        self.pos.push((from, self.g[from].len()));
        let rev = self.g[to].len();
        self.g[from].push(_Edge { to, rev, cap, cost });
        let rev = self.g[from].len() - 1;
        self.g[to].push(_Edge {
            to: from,
            rev,
            cap: T::zero(),
            cost: -cost,
        });
        return self.pos.len() - 1;
    }

    pub fn flow(&mut self, source: usize, sink: usize, flow_limit: T) -> (T, T) {
        self.slope(source, sink, flow_limit).pop().unwrap()
    }

    pub fn slope(&mut self, source: usize, sink: usize, flow_limit: T) -> Vec<(T, T)> {
        let n = self.g.len();
        let mut dual = vec![T::zero(); n];
        let mut prev_v = vec![0; n];
        let mut prev_e = vec![0; n];
        let mut flow = T::zero();
        let mut cost = T::zero();
        let mut prev_cost_per_flow: Option<T> = None;
        let mut result = vec![(flow, cost)];
        while flow < flow_limit {
            if !self.refine_dual(source, sink, &mut dual, &mut prev_v, &mut prev_e) {
                break;
            }
            let mut c = flow_limit - flow;
            let mut v = sink;
            while v != source {
                c = std::cmp::min(c, self.g[prev_v[v]][prev_e[v]].cap);
                v = prev_v[v];
            }
            let mut v = sink;
            while v != source {
                self.g[prev_v[v]][prev_e[v]].cap -= c;
                let rev = self.g[prev_v[v]][prev_e[v]].rev;
                self.g[v][rev].cap += c;
                v = prev_v[v];
            }
            let d = -dual[source];
            flow += c;
            cost += d * c;
            result.push((flow, cost));
            prev_cost_per_flow = Some(d);
        }
        return result;
    }

    fn refine_dual(
        &self,
        source: usize,
        sink: usize,
        dual: &mut [T],
        pv: &mut [usize],
        pe: &mut [usize],
    ) -> bool {
        let n = self.g.len();
        let mut dist = vec![T::max_value(); n];
        let mut vis = vec![false; n];
        let mut que = std::collections::BinaryHeap::new();
        dist[source] = T::zero();
        que.push((std::cmp::Reverse(T::zero()), source));
        while let Some((_, v)) = que.pop() {
            if vis[v] {
                continue;
            }
            vis[v] = true;
            if v == sink {
                break;
            }
            for (i, e) in self.g[v].iter().enumerate() {
                if vis[e.to] || e.cap == T::zero() {
                    continue;
                }
                let cost = e.cost - dual[e.to] + dual[v];
                if dist[e.to] - dist[v] > cost {
                    dist[e.to] = dist[v] + cost;
                    pv[e.to] = v;
                    pe[e.to] = i;
                    que.push((std::cmp::Reverse(dist[e.to]), e.to));
                }
            }
        }
        if !vis[sink] {
            return false;
        }
        for v in 0..n {
            if !vis[v] {
                continue;
            }
            dual[v] -= dist[sink] - dist[v];
        }
        return true;
    }
}

struct _Edge<T> {
    to: usize,
    rev: usize,
    cap: T,
    cost: T,
}

const INF: isize = 1<<40;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: isize,
    }
    let M = N as isize;
    let mut A = Vec::new();
    for _ in 0..N {
        input! {
            a: [isize; N],
        }
        A.push(a);
    }
    let mut MG = MinCostFlowGraph::new(N * 2 + 2);
    let s = N * 2;
    let t = N * 2 + 1;
    MG.add_edge(s, t, M * K, INF);
    for i in 0..N {
        MG.add_edge(s, i, K, 0);
        MG.add_edge(N + i, t, K, 0);
        for j in 0..N {
            MG.add_edge(i, N + j, 1, INF - A[i][j]);
        }
    }
    let res = MG.flow(s, t, M * K);
    println!("{}", M * K * INF - res.1);
    let mut G = vec![vec!['.'; N]; N];
    for e in MG.edges() {
        if e.from == s || e.to == t || e.flow == 0 {
            continue;
        }
        let x = e.from;
        let y = e.to - N;
        G[x][y] = 'X';
    }
    for i in 0..N {
        println!("{}", G[i].iter().collect::<String>());
    }
}