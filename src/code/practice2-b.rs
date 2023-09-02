// https://atcoder.jp/contests/practice2/tasks/practice2_b

use proconio::input;
use proconio::fastout;

use std::ops::{Bound, RangeBounds};
pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
}

impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
    pub fn new(n: usize, e: T) -> Self {
        FenwickTree {
            n,
            ary: vec![e.clone(); n],
            e,
        }
    }

    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        return sum;
    }

    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }

    pub fn sum<R>(&self, range: R) -> T
    where
        T: std::ops::Sub<Output = T>,
        R: RangeBounds<usize>,
    {
        let r = match range.end_bound() {
            Bound::Included(r) => r + 1,
            Bound::Excluded(r) => *r,
            Bound::Unbounded => self.n,
        };
        let l = match range.start_bound() {
            Bound::Included(l) => *l,
            Bound::Excluded(l) => l + 1,
            Bound::Unbounded => return self.accum(r),
        };
        return self.accum(r) - self.accum(l);
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        a: [usize; N],
    }
    let mut FT = FenwickTree::new(N, 0);
    for i in 0..N {
        FT.add(i, a[i]);
    }
    for _ in 0..Q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                x: usize,
            }
            FT.add(p, x);
        }
        else {
            input! {
                l: usize,
                r: usize,
            }
            println!("{}", FT.sum(l..r));
        }
    }
}