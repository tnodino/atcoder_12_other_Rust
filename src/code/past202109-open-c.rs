// https://atcoder.jp/contests/past202109-open/tasks/past202109_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N],
    }
    println!("{}", A.iter().filter(|&x| *x == X).count());
}