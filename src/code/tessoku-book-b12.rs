// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ck

use proconio::input;
use proconio::fastout;
use libm::pow;

fn f(x: f64) -> f64 {
    return pow(x, 3.) + x;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: f64,
    }
    let mut ok = N;
    let mut ng = 0.;
    for _ in 0..100 {
        let mid = (ok + ng) / 2.;
        if f(mid) >= N {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}