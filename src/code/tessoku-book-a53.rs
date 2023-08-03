// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ba

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut bh = BinaryHeap::new();
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: isize,
            }
            bh.push(-x);
        }
        else if q == 2 {
            let x = -bh.pop().unwrap();
            println!("{}", x);
            bh.push(-x);
        }
        else {
            bh.pop().unwrap();
        }
    }
}