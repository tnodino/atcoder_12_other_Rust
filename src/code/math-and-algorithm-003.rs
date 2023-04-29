// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    println!("{}", A.iter().sum::<usize>());
}