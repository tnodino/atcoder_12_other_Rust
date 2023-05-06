// https://atcoder.jp/contests/qupc2014/tasks/qupc2014_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
        D: usize,
        E: [[usize; A]; C],
    }
    for s in (0..=100).rev() {
        let mut cnt1 = 0;
        for i in 0..C {
            let mut cnt2 = 0;
            for j in 0..A {
                if E[i][j] >= s {
                    cnt2 += 1;
                }
            }
            if cnt2 >= B {
                cnt1 += 1;
            }
        }
        if cnt1 >= D {
            println!("{}", s);
            return;
        }
    }
}