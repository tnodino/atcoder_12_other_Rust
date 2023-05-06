// https://atcoder.jp/contests/past202209-open/tasks/past202209_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        P1: [f64; 6],
        P2: [f64; 6],
        P3: [f64; 6],
    }
    let mut P = [0.; 20];
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                P[i+j+k+3] += P1[i] * P2[j] * P3[k];
            }
        }
    }
    for i in 1..=18 {
        println!("{}", P[i] / 1_000_000.);
    }
}