// https://atcoder.jp/contests/past202109-open/tasks/past202109_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        Y: usize,
    }
    let mut cntx = 0;
    let mut cnty = 0;
    for i in 1..=X {
        if X % i == 0 {
            cntx += 1;
        }
    }
    for i in 1..=Y {
        if Y % i == 0 {
            cnty += 1;
        }
    }
    if cntx > cnty {
        println!("X");
    }
    else if cntx < cnty {
        println!("Y");
    }
    else {
        println!("Z");
    }
}