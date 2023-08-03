// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_u

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let mut res = 1;
    for i in 1..=r {
        res *= n - i + 1;
        res /= i;
    }
    println!("{}", res);
}