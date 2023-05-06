// https://atcoder.jp/contests/past202203-open/tasks/past202203_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        alp: String,
    }
    let mut N = alp.len() / 3;
    let mut M = alp.len() % 3;
    if M == 0 {
        N -= 1;
        M += 3;
    }
    let alp = alp.chars().collect::<Vec<char>>();
    for i in 0..M {
        print!("{}", alp[i]);
    }
    println!("{}", (N as u8 + 96) as char);
}