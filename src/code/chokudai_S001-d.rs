// https://atcoder.jp/contests/chokudai_S001/tasks/chokudai_S001_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut a: [usize; N],
    }
    a.sort();
    println!("{}", a.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}