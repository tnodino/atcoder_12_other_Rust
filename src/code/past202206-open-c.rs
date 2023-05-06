// https://atcoder.jp/contests/past202206-open/tasks/past202206_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let X = 1_000_000_000;
    let mut x = 1;
    for _ in 0..M {
        x *= N;
        if x <= X {
            print!("o");
        }
        else {
            x /= N;
            print!("x");
        }
    }
    println!();
}