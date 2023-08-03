// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_o

use proconio::input;
use proconio::fastout;

use std::cmp::Ordering;
fn bisect_right<T: Ord>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x <= v {
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
        A: [usize; N],
    }
    let mut vec = A.clone();
    vec.sort();
    vec.dedup();
    let mut ans = Vec::new();
    for i in 0..N {
        ans.push(bisect_right(&vec, &A[i]));
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}