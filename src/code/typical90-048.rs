// https://atcoder.jp/contests/typical90/tasks/typical90_av

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
        }
        vec.push(B);
        vec.push(A-B);
    }
    vec.sort_by(|a, b| b.cmp(&a));
    println!("{}", vec[..K].iter().sum::<usize>());
}