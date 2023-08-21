// https://atcoder.jp/contests/njpc2017/tasks/njpc2017_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        S: String,
    }
    if S.len() < L {
        println!("{}", S);
    }
    else {
        println!("{}", &S[..L]);
    }
}