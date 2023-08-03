// https://atcoder.jp/contests/tessoku-book/tasks/math_and_algorithm_al

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        T: usize,
        N: usize,
    }
    let mut S = vec![0; T+1];
    for _ in 0..N {
        input! {
            L: usize,
            R: usize,
        }
        S[L] += 1;
        S[R] -= 1;
    }
    for i in 1..T {
        S[i] += S[i-1];
    }
    for i in 0..T {
        println!("{}", S[i]);
    }
}