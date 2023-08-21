// https://atcoder.jp/contests/dp/tasks/dp_q

const E: usize = 0;

use std::cmp::max;
fn op(a: usize, b: usize) -> usize {
    return max(a, b);
}

struct SegmentTree<D, Op> {
    e: D,
    op: Op,
    sz: usize,
    d: Vec<D>,
}

impl<D: Copy, Op: Fn(D, D) -> D> SegmentTree<D, Op>  {
    fn new(v: &[D], e: D, op: Op) -> Self {
        let n = v.len();
        let mut sz = 1;
        while sz < n {
            sz <<= 1;
        }
        let mut d = vec![e; sz*2];
        for i in 0..n {
            d[sz+i] = v[i];
        }
        for i in (0..sz).rev() {
            d[i] = op(d[i*2], d[i*2+1]);
        }
        SegmentTree {
            e,
            op,
            sz,
            d,
        }
    }

    fn update(&mut self, mut p: usize, x: D) {
        p += self.sz;
        self.d[p] = x;
        while p >= 2 {
            p >>= 1;
            self.d[p] = (self.op)(self.d[p*2], self.d[p*2+1]);
        }
    }

    fn query(&self, mut a: usize, mut b: usize) -> D {
        let mut sml = self.e;
        let mut smr = self.e;
        a += self.sz;
        b += self.sz;
        while a < b {
            if a & 1 > 0 {
                sml = (self.op)(sml, self.d[a]);
                a += 1;
            }
            if b & 1 > 0 {
                b -= 1;
                smr = (self.op)(self.d[b], smr);
            }
            a >>= 1;
            b >>= 1;
        }
        return (self.op)(sml, smr);
    }
}

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        h: [usize; N],
        a: [usize; N],
    }
    let vec = vec![0; N+1];
    let mut DP = SegmentTree::new(&vec, E, op);
    for i in 0..N {
        let res = DP.query(0, h[i]) + a[i];
        DP.update(h[i], res);
    }
    println!("{}", DP.query(0, N+1));
}