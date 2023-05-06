// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_fc

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    N -= 1;
    let mut ans = Vec::new();
    for _ in 0..10 {
        if N & 1 > 0 {
            ans.push(7);
        }
        else {
            ans.push(4);
        }
        N >>= 1;
    }
    for i in (0..10).rev() {
        print!("{}", ans[i]);
    }
    println!();
}