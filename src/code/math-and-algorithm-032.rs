// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ad

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
        X: usize,
        mut A: [usize; N],
    }
    A.sort();
    A.push(1<<60);
    let idx = bisect_left(&A, &X);
    if A[idx] == X {
        println!("Yes");
    }
    else {
        println!("No");
    }
}