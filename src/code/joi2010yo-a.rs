// https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: usize,
        A: [usize; 9],
    }
    println!("{}", S - A.iter().sum::<usize>());
}