// https://atcoder.jp/contests/codequeen2023-final-open/tasks/codequeen2023_final_b

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

const DX: &[usize] = &[!0, !0, 1, 1];
const DY: &[usize] = &[!0, 1, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut set = HashSet::new();
    let mut flgr = vec![false; N];
    let mut flgc = vec![false; N];
    for _ in 0..N-1 {
        input! {
            r: usize,
            c: usize,
        }
        set.insert((r-1, c-1));
        flgr[r-1] = true;
        flgc[c-1] = true;
    }
    let mut r = 0;
    let mut c = 0;
    for i in 0..N {
        if !flgr[i] {
            r = i;
        }
        if !flgc[i] {
            c = i;
        }
    }
    for d in 0..4 {
        let mut x = r;
        let mut y = c;
        loop {
            let nx = x.wrapping_add(DX[d]);
            let ny = y.wrapping_add(DY[d]);
            if N <= nx || N <= ny {
                break;
            }
            if set.contains(&(nx, ny)) {
                println!("-1");
                return;
            }
            x = nx;
            y = ny;
        }
    }
    println!("{} {}", r + 1, c + 1);
}