// https://atcoder.jp/contests/code-festival-2014-final/tasks/code_festival_final_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: f64,
    }
    println!("{}", 50. / s);
}