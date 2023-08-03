// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ee

use proconio::input;
use proconio::fastout;

const INF: usize = 1<<60;

use std::cmp::min;
struct SegmentTree {
    sz: usize,
    d: Vec<usize>,
}

impl SegmentTree  {
    fn new(v: &[usize]) -> Self {
        let n = v.len();
        let mut sz = 1;
        while sz < n {
            sz <<= 1;
        }
        let mut d = vec![INF; sz*2];
        for i in 0..n {
            d[sz+i] = v[i];
        }
        for i in (0..sz).rev() {
            d[i] = min(d[i*2], d[i*2+1]);
        }
        SegmentTree {
            sz,
            d,
        }
    }

    fn update(&mut self, mut p: usize, x: usize) {
        p += self.sz;
        self.d[p] = x;
        while p >= 2 {
            p >>= 1;
            self.d[p] = min(self.d[p*2], self.d[p*2+1]);
        }
    }

    fn query(&self, mut a: usize, mut b: usize) -> usize {
        let mut sml = INF;
        let mut smr = INF;
        a += self.sz;
        b += self.sz;
        while a < b {
            if a & 1 > 0 {
                sml = min(sml, self.d[a]);
                a += 1;
            }
            if b & 1 > 0 {
                b -= 1;
                smr = min(self.d[b], smr);
            }
            a >>= 1;
            b >>= 1;
        }
        return min(sml, smr);
    }
}

use std::cmp::Ordering;
fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        L: isize,
        R: isize,
        X: [isize; N],
    }
    let mut DP = vec![INF; N];
    DP[0] = 0;
    let mut Seg = SegmentTree::new(&DP);
    for i in 1..N {
        let l = bisect_left(&X, &(X[i]-R));
        let r = bisect_left(&X, &(X[i]-L+1));
        DP[i] = Seg.query(l, r) + 1;
        Seg.update(i, DP[i]);
    }
    println!("{}", DP[N-1]);
}