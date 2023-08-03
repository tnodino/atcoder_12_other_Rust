// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bi

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
    for i in 1..=N {
        println!("{}: {}{}{}", i, '{', G[i].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", "), '}');
    }
}