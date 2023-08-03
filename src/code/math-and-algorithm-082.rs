// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_bn

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            L: usize,
            R: usize,
        }
        vec.push((L, R));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    let mut ans = 0;
    let mut r = 0;
    for i in 0..N {
        if r <= vec[i].0 {
            ans += 1;
            r = vec[i].1;
        }
    }
    println!("{}", ans);
}