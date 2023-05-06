// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_n

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    let M = sqrt(N as f64) as usize;
    let mut vec = Vec::new();
    for i in 2..=M {
        while N % i == 0 {
            vec.push(i);
            N /= i;
        }
    }
    if N > 1 {
        vec.push(N);
    }
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}