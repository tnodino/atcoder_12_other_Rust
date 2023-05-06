// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_r

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt: [usize; 4] = [0; 4];
    for i in 0..N {
        cnt[match A[i] {
            100 => 0,
            200 => 1,
            300 => 2,
            _ => 3,
        }] += 1;
    }
    println!("{}", cnt[0] * cnt[3] + cnt[1] * cnt[2]);
}