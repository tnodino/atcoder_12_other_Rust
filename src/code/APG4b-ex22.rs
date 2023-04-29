// https://atcoder.jp/contests/APG4b/tasks/APG4b_ca

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        vec.push((a, b));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    for (a, b) in vec {
        println!("{} {}", a, b);
    }
}