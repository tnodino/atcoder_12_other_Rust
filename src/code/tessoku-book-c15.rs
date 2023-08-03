// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fm

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let M = 200_000;
    let mut L = Vec::new();
    let mut R = Vec::new();
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            l: usize,
            r: usize,
        }
        L.push(l);
        R.push(r+K);
        vec.push((l, r+K));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut cntl = vec![0; M];
    let mut idx = 0;
    for i in 0..N {
        if idx <= vec[i].0 {
            cntl[vec[i].1] += 1;
            idx = vec[i].1;
        }
    }
    for i in 0..M-1 {
        cntl[i+1] += cntl[i];
    }
    vec.sort_by(|a, b| b.0.cmp(&a.0));
    let mut cntr = vec![0; M];
    let mut idx = M;
    for i in 0..N {
        if vec[i].1 <= idx {
            cntr[vec[i].0] += 1;
            idx = vec[i].0;
        }
    }
    for i in (1..M).rev() {
        cntr[i-1] += cntr[i];
    }
    for i in 0..N {
        println!("{}", cntl[L[i]] + cntr[R[i]] + 1);
    }
}