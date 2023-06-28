// https://atcoder.jp/contests/joi2007yo/tasks/joi2007yo_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: [usize; 4],
        B: [usize; 4],
    }
    let A = A.iter().sum::<usize>();
    let B = B.iter().sum::<usize>();
    if A >= B {
        println!("{}", A);
    }
    else {
        println!("{}", B);
    }
}