// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bb

use proconio::input;
use proconio::fastout;
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: String,
                y: usize,
            }
            map.insert(x, y);
        }
        else {
            input! {
                x: String,
            }
            println!("{}", map[&x]);
        }
    }
}