// https://atcoder.jp/contests/past202206-open/tasks/past202206_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        A: usize,
        B: usize,
        C: usize,
    }
    let har = A * B * C + B * X;
    let tor = A * X;
    if har < tor {
        println!("Hare");
    }
    else if har > tor {
        println!("Tortoise");
    }
    else {
        println!("Tie");
    }
}