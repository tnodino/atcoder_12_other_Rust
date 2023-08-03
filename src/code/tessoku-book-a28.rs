// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ab

use proconio::input;
use proconio::fastout;

const MOD: usize = 10_000;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut ans = 0;
    for _ in 0..N {
        input! {
            T: char,
            A: usize,
        }
        match T {
            '+' => ans += A,
            '-' => ans = (ans + MOD) - A,
            _ => ans *= A,
        }
        ans %= MOD;
        println!("{}", ans);
    }
}