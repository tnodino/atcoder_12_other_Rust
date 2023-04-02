// https://atcoder.jp/contests/abc049/tasks/abc049_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        c: char,
    }
    if "aeiou".contains(c) {
        println!("vowel");
    }
    else {
        println!("consonant");
    }
}