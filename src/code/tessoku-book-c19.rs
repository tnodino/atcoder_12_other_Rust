// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fq

#[allow(non_snake_case)]
struct SimpleSeg<D, Op> {
    e: D,
    op: Op,
    sz: usize,
    d: Vec<D>,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl<D: Copy, Op: Fn(D, D) -> D> SimpleSeg<D, Op> {
    fn new(v: &[D], e: D, op: Op) -> Self {
        let n = v.len();
        let mut lg = 1;
        while (1 << lg) < n {
            lg += 1;
        }
        let sz = 1 << lg;
        let mut d = vec![e; 2*sz];
        for i in 0..n {
            d[sz+i] = v[i];
        }
        for i in (0..sz).rev() {
            d[i] = op(d[2*i], d[2*i+1]);
        }
        SimpleSeg {
            e,
            op,
            sz,
            d,
        }
    }

    fn sum(&self, mut a: usize, mut b: usize) -> D {
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
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: usize,
        K: usize,
    }
    let mut mi: Vec<usize> = vec![1<<60; L+1];
    for _ in 0..N {
        input! {
            A: usize,
            C: usize,
        }
        mi[A-1] = min(mi[A-1], C);
    }
    let Seg = SimpleSeg::new(&mi, 1<<60, |l, r| {
        return min(l, r);
    });
    let mut ans = 0;
    for i in 0..L-K {
        let res = Seg.sum(i, i + K);
        if res == 1 << 60 {
            println!("-1");
            return;
        }
        ans += res;
    }
    println!("{}", ans);
}