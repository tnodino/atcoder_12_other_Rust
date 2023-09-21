// https://atcoder.jp/contests/practice2/tasks/practice2_b

use proconio::input;
use proconio::fastout;
use ac_library::FenwickTree;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
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
        match t {
            0 => {
                input! {
                    (p, x): (usize, usize),
                }
                FT.add(p, x);
            },
            1 => {
                input! {
                    (l, r): (usize, usize),
                }
                println!("{}", FT.sum(l..r));
            },
            _ => {},
        }
    }
}