// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ag

use proconio::input;
use proconio::fastout;
use libm::hypot;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        x1: f64,
        y1: f64,
        r1: f64,
        x2: f64,
        y2: f64,
        r2: f64,
    }
    let dist = hypot(x1 - x2, y1 - y2);
    if dist < (r1 - r2).abs() {
        println!("1");
    }
    else if dist == (r1 - r2).abs() {
        println!("2");
    }
    else if dist < r1 + r2 {
        println!("3");
    }
    else if dist == r1 + r2 {
        println!("4");
    }
    else {
        println!("5");
    }
}