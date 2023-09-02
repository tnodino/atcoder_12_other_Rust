// https://atcoder.jp/contests/NYC2015/tasks/nyc2015_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let mut vec = Vec::new();
    while N > 0 {
        vec.push(N % 2);
        N /= 2;
    }
    let M = vec.len();
    for i in 0..M {
        if vec[i] != vec[M-i-1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}