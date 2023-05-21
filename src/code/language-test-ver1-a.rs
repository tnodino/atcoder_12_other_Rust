// https://atcoder.jp/contests/language-test-ver1/tasks/test001_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        c: String,
    }
    let mut cnt = vec![0; 4];
    for x in c.chars() {
        let idx = x as usize - 49;
        cnt[idx] += 1;
    }
    println!("{} {}", cnt.iter().max().unwrap(), cnt.iter().min().unwrap());
}