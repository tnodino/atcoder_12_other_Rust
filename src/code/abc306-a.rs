// https://atcoder.jp/contests/abc306/tasks/abc306_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    for i in 0..N {
        print!("{}", &S[i..i+1].repeat(2));
    }
    println!();
}