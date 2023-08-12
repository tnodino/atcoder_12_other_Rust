// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_bd

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
    }
    let mut ans = N * N * N;
    for b in 1..=N {
        let l;
        if K - 1 < b {
            l = b + 1 - K;
        }
        else {
            l = 1;
        }
        let r;
        if b + K - 1 <= N {
            r = b + K - 1;
        }
        else {
            r = N;
        }
        for w in l..=r {
            for g in l..=r {
                let x;
                let y;
                if w <= g {
                    x = w;
                    y = g;
                }
                else {
                    x = g;
                    y = w;
                }
                if y - x <= K - 1 {
                    ans -= 1;
                }
            }
        }
    }
    println!("{}", ans);
}