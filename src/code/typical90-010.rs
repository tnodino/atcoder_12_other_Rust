// https://atcoder.jp/contests/typical90/tasks/typical90_j

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut s1 = vec![0; N+1];
    let mut s2 = vec![0; N+1];
    for i in 1..=N {
        input! {
            C: usize,
            P: usize,
        }
        s1[i] += s1[i-1];
        s2[i] += s2[i-1];
        if C == 1 {
            s1[i] += P;
        }
        else {
            s2[i] += P;
        }
    }
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            L: usize,
            R: usize,
        }
        println!("{} {}", s1[R] - s1[L-1], s2[R] - s2[L-1]);
    }
}