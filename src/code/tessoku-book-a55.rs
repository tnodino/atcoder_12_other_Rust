// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bc

use proconio::input;
use proconio::fastout;
use std::collections::BTreeSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut set = BTreeSet::new();
    for _ in 0..Q {
        input! {
            q: usize,
            x: isize,
        }
        if q == 1 {
            set.insert(x);
        }
        else if q == 2 {
            set.remove(&x);
        }
        else {
            println!("{}", set.range(x..).next().unwrap_or(&(-1)));
        }
    }
}