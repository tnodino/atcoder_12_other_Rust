// https://atcoder.jp/contests/typical90/tasks/typical90_ag

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    if H == 1 || W == 1 {
        println!("{}", H * W);
    }
    else {
        println!("{}", ((H + 1) / 2) * ((W + 1) / 2));
    }
}