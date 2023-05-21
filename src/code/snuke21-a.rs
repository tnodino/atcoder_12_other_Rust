// https://atcoder.jp/contests/snuke21/tasks/snuke21_a

use proconio::input;
use proconio::fastout;
use libm::sqrt;

fn f(x: usize) -> usize {
    return (x + 1) * x / 2;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
    }
    let m = sqrt(n as f64) as usize;
    let mut ok = 1;
    let mut ng = m * 2 + 1;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid) <= n {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    if f(ok) == n {
        println!("{}", ok);
    }
    else {
        println!("-1");
    }
}