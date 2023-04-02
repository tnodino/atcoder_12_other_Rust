// https://atcoder.jp/contests/APG4b/tasks/APG4b_co

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        p: usize,
    }
    if p == 1 {
        input! {
            price: usize,
            N: usize,
        }
        println!("{}", N * price);
    }
    else {
        input! {
            text: String,
            price: usize,
            N: usize,
        }
        println!("{}!", text);
        println!("{}", N * price);
    }
}