// https://atcoder.jp/contests/past202004-open/tasks/past202004_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
        T: String,
    }
    let mut vec = Vec::new();
    for i in (1..=9).rev() {
        vec.push(format!("B{}", i));
    }
    for i in 1..=9 {
        vec.push(format!("{}F", i));
    }
    let mut s = 0;
    let mut t = 0;
    for i in 0..vec.len() {
        if vec[i] == S {
            s = i as isize;
        }
        if vec[i] == T {
            t = i as isize;
        }
    }
    println!("{}", (s - t).abs());
}