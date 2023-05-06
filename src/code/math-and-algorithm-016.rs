// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_p

use proconio::input;
use proconio::fastout;
use num::integer::gcd;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = A[0];
    for a in A {
        ans = gcd(ans, a);
    }
    println!("{}", ans);
}