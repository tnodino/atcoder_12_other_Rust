// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dn

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut X: usize,
        mut Y: usize,
    }
    let mut ans = Vec::new();
    while X > 1 || Y > 1 {
        ans.push((X, Y));
        if X >= Y {
            X -= Y;
        }
        else {
            Y -= X;
        }
    }
    println!("{}", ans.len());
    for i in (0..ans.len()).rev() {
        println!("{} {}", ans[i].0, ans[i].1);
    }
}