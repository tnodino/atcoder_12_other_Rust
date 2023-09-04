// https://atcoder.jp/contests/practice2/tasks/practice2_j

use proconio::input;
use proconio::fastout;
use ac_library::{Segtree, Max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
        A: [isize; N],
    }
    let mut Seg = Segtree::<Max<isize>>::new(N+1);
    for i in 1..=N {
        Seg.set(i, A[i-1]);
    }
    for _ in 0..Q {
        input! {
            T: usize,
        }
        match T {
            1 => {
                input! {
                    X: usize,
                    V: isize,
                }
                Seg.set(X, V);
            },
            2 => {
                input! {
                    L: usize,
                    R: usize,
                }
                println!("{}", Seg.prod(L..=R));
            },
            3 => {
                input! {
                    X: usize,
                    V: isize,
                }
                println!("{}", Seg.max_right(X, |a| a < &V));
            },
            _ => {},
        }
    }
}