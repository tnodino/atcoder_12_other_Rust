// https://atcoder.jp/contests/typical90/tasks/typical90_bz

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut cnt = vec![0; N];
    for _ in 0..M {
        input! {
            a: usize,
            b: usize,
        }
        if a > b {
            cnt[a-1] += 1;
        }
        else {
            cnt[b-1] += 1;
        }
    }
    println!("{}", cnt.iter().filter(|&x| *x == 1).count());
}