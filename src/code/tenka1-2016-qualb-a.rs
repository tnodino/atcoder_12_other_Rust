// https://atcoder.jp/contests/tenka1-2016-qualb/tasks/tenka1_2016_qualB_a

use proconio::fastout;

fn f(n: usize) -> usize {
    return (n * n + 4) / 8;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 20;
    println!("{}", f(f(f(N))));
}