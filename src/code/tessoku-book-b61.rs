// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_eh

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N+1];
    for _ in 0..M {
        input! {
            A: usize,
            B: usize,
        }
        G[A].push(B);
        G[B].push(A);
    }
    let mut ma = 0;
    let mut ans = 0;
    for i in 1..=N {
        if ma < G[i].len() {
            ma = G[i].len();
            ans = i;
        }
    }
    println!("{}", ans);
}