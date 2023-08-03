// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_l

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, A: &Vec<usize>) -> usize {
    let mut cnt = 0;
    for a in A.iter() {
        cnt += x / a;
    }
    return cnt;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize; N],
    }
    let mut ok = 1<<32;
    let mut ng = 0;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, &A) >= K {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}