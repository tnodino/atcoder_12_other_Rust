// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_bb

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    if N % 2 == 0 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}