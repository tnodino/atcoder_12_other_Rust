// https://atcoder.jp/contests/past202010-open/tasks/past202010_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    if Y == 0 {
        println!("ERROR");
    }
    else {
        let X = X * 100;
        let res = (X / Y) as f64 / 100.;
        println!("{:.2}", res);
    }
}