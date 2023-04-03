// https://atcoder.jp/contests/abc163/tasks/abc163_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; M],
    }
    let a = A.iter().sum::<usize>();
    if N >= a {
        println!("{}", N - a);
    }
    else {
        println!("-1");
    }
}