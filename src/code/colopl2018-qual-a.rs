// https://atcoder.jp/contests/colopl2018-qual/tasks/colopl2018_qual_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        S: String,
    }
    let N = S.len();
    if A <= N && N <= B {
        println!("YES");
    }
    else {
        println!("NO");
    }
}