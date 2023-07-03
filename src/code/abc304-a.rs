// https://atcoder.jp/contests/abc304/tasks/abc304_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x = 0;
    let mut age = 1<<32;
    let mut S = Vec::new();
    for i in 0..N {
        input! {
            s: String,
            A: usize,
        }
        S.push(s);
        if A < age {
            x = i;
            age = A;
        }
    }
    for i in x..x+N {
        println!("{}", S[i%N]);
    }
}