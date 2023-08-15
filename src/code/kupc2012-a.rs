// https://atcoder.jp/contests/kupc2012/tasks/kupc2012_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        T: usize,
        E: usize,
        x: [usize; N],
    }
    for i in 0..N {
        for j in 1..=1440 {
            if T - E <= x[i] * j && x[i] * j <= T + E {
                println!("{}", i + 1);
                return;
            }
        }
    }
    println!("-1");
}