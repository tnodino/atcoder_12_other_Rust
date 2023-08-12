// https://atcoder.jp/contests/joi2007yo/tasks/joi2007yo_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut vec = (1..=n*2).collect::<Vec<usize>>();
    for _ in 0..m {
        input! {
            k: usize,
        }
        let mut vec2 = vec![0; n*2];
        if k == 0 {
            for i in 0..n {
                vec2[i*2] = vec[i];
                vec2[i*2+1] = vec[n+i];
            }
        }
        else {
            for i in k..n*2 {
                vec2[i-k] = vec[i];
            }
            for i in 0..k {
                vec2[n*2-k+i] = vec[i];
            }
        }
        vec = vec2;
    }
    for i in 0..n*2 {
        println!("{}", vec[i]);
    }
}