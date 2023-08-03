// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cj

use proconio::input;
use proconio::fastout;

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
        mut A: [usize; N],
        Q: usize,
    }
    A.sort();
    for _ in 0..Q {
        input! {
            X: usize,
        }
        println!("{}", bisect_left(&A, &X));
    }
}