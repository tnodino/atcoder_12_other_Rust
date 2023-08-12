// https://atcoder.jp/contests/joi2007yo/tasks/joi2007yo_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        s: String,
    }
    let n = s.len();
    let s = s.chars().collect::<Vec<char>>();
    for i in 0..n {
        let mut x = (s[i] as u8) - ('A' as u8);
        x = (x + 23) % 26;
        print!("{}", (('A' as u8) + x) as char);
    }
    println!();
}