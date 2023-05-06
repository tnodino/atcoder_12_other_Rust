// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fa

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        D: usize,
        X: isize,
    }
    let mut vec = vec![X];
    for i in 0..D-1 {
        input! {
            A: isize,
        }
        vec.push(vec[i] + A);
    }
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            S: usize,
            T: usize,
        }
        if vec[S-1] > vec[T-1] {
            println!("{}", S);
        }
        else if vec[S-1] < vec[T-1] {
            println!("{}", T);
        }
        else {
            println!("Same");
        }
    }
}