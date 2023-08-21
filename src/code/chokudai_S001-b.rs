// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    println!("{}", a.iter().sum::<usize>());
}