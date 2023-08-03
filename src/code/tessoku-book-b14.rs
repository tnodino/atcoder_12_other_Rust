// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_cm

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
    }
    let mut X = Vec::new();
    let mut Y = Vec::new();
    for i in 0..N {
        if i % 2 == 0 {
            X.push(A[i]);
        }
        else {
            Y.push(A[i]);
        }
    }
    let M = X.len();
    let mut vec1 = Vec::new();
    for bit in 0..1<<M {
        let mut res = 0;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                res += X[i];
            }
        }
        vec1.push(res);
    }
    let M = Y.len();
    let mut vec2 = Vec::new();
    for bit in 0..1<<M {
        let mut res = 0;
        for i in 0..M {
            if bit & (1 << i) > 0 {
                res += Y[i];
            }
        }
        vec2.push(res);
    }
    vec2.push(K+1);
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