// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_af

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for _ in 0..N {
        input! {
            x: f64,
            y: f64,
        }
        vec.push((x, y));
    }
    let mut ans = 10_000_000.;
    for i in 0..N {
        for j in i+1..N {
            let dist = hypot(vec[i].0 - vec[j].0, vec[i].1 - vec[j].1);
            if dist < ans {
                ans = dist;
            }
        }
    }
    println!("{}", ans);
}