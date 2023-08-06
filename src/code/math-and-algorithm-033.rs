// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_ae

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        cx: f64,
        cy: f64,
    }
    let bax = ax - bx;
    let bay = ay - by;
    let bcx = cx - bx;
    let bcy = cy - by;
    let cax = ax - cx;
    let cay = ay - cy;
    let cbx = bx - cx;
    let cby = by - cy;
    if bax * bcx + bay * bcy < 0. {
        println!("{}", sqrt(bax * bax + bay * bay))
    }
    else if cax * cbx + cay * cby < 0. {
        println!("{}", sqrt(cax * cax + cay * cay))
    }
    else {
        let s = (bax * cay - bay * cax).abs();
        let bc = sqrt(bcx * bcx + bcy * bcy);
        println!("{}", s / bc);
    }
}