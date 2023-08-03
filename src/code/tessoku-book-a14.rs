// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_n

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
        K: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
        D: [usize; N],
    }
    let mut vec1 = Vec::new();
    for i in 0..N {
        for j in 0..N {
            vec1.push(A[i] + B[j]);
        }
    }
    let mut vec2 = Vec::new();
    for i in 0..N {
        for j in 0..N {
            vec2.push(C[i] + D[j]);
        }
    }
    vec2.push(K);
    vec2.sort();
    for v in vec1.into_iter() {
        if v > K {
            continue;
        }
        if vec2[bisect_left(&vec2, &(K-v))] == K - v {
            println!("Yes");
            return;
        }
    }
    println!("No");
}