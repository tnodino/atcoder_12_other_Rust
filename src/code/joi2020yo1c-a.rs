// https://atcoder.jp/contests/joi2020yo1c/tasks/joi2020_yo1c_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: usize,
        L: usize,
        R: usize,
    }
    if X <= L {
        println!("{}", L);
    }
    else if R <= X {
        println!("{}", R);
    }
    else {
        println!("{}", X);
    }
}