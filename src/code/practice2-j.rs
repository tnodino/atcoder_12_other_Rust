// https://atcoder.jp/contests/practice2/tasks/practice2_j

use proconio::input;
use proconio::fastout;
use ac_library::{Segtree, Max};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, Q): (usize, usize),
        A: [isize; N],
    }
    let mut Seg = Segtree::<Max<isize>>::new(N + 1);
    for i in 0..N {
        Seg.set(i + 1, A[i]);
    }
    for _ in 0..Q {
        input! {
            T: usize,
        }
        match T {
            1 => {
                input! {
                    (X, V): (usize, isize),
                }
                Seg.set(X, V);
            },
            2 => {
                input! {
                    (L, R): (usize, usize),
                }
                println!("{}", Seg.prod(L..=R));
            },
            3 => {
                input! {
                    (X, V): (usize, isize),
                }
                println!("{}", Seg.max_right(X, |x| x < &V));
            },
            _ => {},
        }
    }
}