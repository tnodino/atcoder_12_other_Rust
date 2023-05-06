// https://atcoder.jp/contests/joi2007yo/tasks/joi2007yo_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let mut flg = [false; 31];
    for _ in 0..28 {
        input! {
            A: usize,
        }
        flg[A] = true;
    }
    for i in 1..=30 {
        if !flg[i] {
            println!("{}", i);
        }
    }
}