// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_s

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = 0;
    let mut cnt: Vec<usize> = vec![0; 3];
    for i in 0..N {
        cnt[A[i]-1] += 1;
    }
    for i in 0..3 {
        if cnt[i] >= 2 {
            ans += (cnt[i] - 1) * cnt[i] / 2;
        }
    }
    println!("{}", ans);
}