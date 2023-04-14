// https://atcoder.jp/contests/abc110/tasks/abc110_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        X: isize,
        Y: isize,
        x: [isize; N],
        y: [isize; M],
    }
    let ma = *x.iter().max().unwrap();
    let mi = *y.iter().min().unwrap();
    for z in X+1..=Y {
        if ma < z && z <= mi {
            println!("No War");
            return;
        }
    }
    println!("War");
}