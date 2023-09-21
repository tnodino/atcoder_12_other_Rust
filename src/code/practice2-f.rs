// https://atcoder.jp/contests/practice2/tasks/practice2_f

use proconio::input;
use proconio::fastout;
use ac_library::{convolution, ModInt998244353 as Mint};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (N, M): (usize, usize),
        a: [Mint; N],
        b: [Mint; M],
    }
    println!("{}", convolution(&a, &b).iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}