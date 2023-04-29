// https://atcoder.jp/contests/past202010-open/tasks/past202010_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    }
    let mut arr = [A, B, C];
    arr.sort();
    if arr[1] == A {
        println!("A");
    }
    else if arr[1] == B {
        println!("B");
    }
    else {
        println!("C");
    }
}