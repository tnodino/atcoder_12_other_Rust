// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_k

use proconio::input;
use proconio::fastout;

fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    return vec.binary_search_by(|x| x.cmp(v)).unwrap_or_else(|x| x);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    println!("{}", bisect_left(&A, &X) + 1);
}