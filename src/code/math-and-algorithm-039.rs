// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_aj

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        Q: usize,
    }
    let mut imos = vec![0; N+1];
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
            X: isize,
        }
        imos[L-1] += X;
        imos[R] -= X;
    }
    for i in 0..N-1 {
        imos[i+1] += imos[i];
        if imos[i] > imos[i+1] {
            print!(">");
        }
        else if imos[i] < imos[i+1] {
            print!("<");
        }
        else {
            print!("=");
        }
    }
    println!();
}