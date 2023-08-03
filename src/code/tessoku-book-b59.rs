// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ef

use proconio::input;
use proconio::fastout;

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
        let mut d = vec![0; sz*2];
        for i in 0..n {
            d[sz+i] = v[i];
        }
        for i in (0..sz).rev() {
            d[i] = d[i*2] + d[i*2+1];
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
            self.d[p] = self.d[p*2] + self.d[p*2+1];
        }
    }

    fn query(&self, mut a: usize, mut b: usize) -> usize {
        let mut sml = 0;
        let mut smr = 0;
        a += self.sz;
        b += self.sz;
        while a < b {
            if a & 1 > 0 {
                sml = sml + self.d[a];
                a += 1;
            }
            if b & 1 > 0 {
                b -= 1;
                smr = self.d[b] + smr;
            }
            a >>= 1;
            b >>= 1;
        }
        return sml + smr;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut Seg = SegmentTree::new(&(vec![0; N]));
    let mut ans = 0;
    for i in (0..N).rev() {
        ans += Seg.query(0, A[i]);
        Seg.update(A[i]-1, 1);
    }
    println!("{}", ans);
}