// https://atcoder.jp/contests/arc001/tasks/arc001_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        _N: usize,
        c: String,
    }
    let c = c.chars().collect::<Vec<char>>();
    let mut cnt = [0; 4];
    for i in 1..=4 {
        let k = (i as u8 + 48) as char;
        cnt[i-1] = c.iter().filter(|&x| *x == k).count();
    }
    println!("{} {}", cnt.iter().max().unwrap(), cnt.iter().min().unwrap());
}