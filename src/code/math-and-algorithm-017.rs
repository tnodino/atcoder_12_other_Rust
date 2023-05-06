// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_q

use proconio::input;
use proconio::fastout;
use num::integer::lcm;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 1;
    for a in A {
        ans = lcm(ans, a);
    }
    println!("{}", ans);
}