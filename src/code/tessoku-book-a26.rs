// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_z

use proconio::input;
use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    for _ in 0..Q {
        input! {
            X: usize,
        }
        let M = sqrt(X as f64) as usize;
        let mut ans = "Yes";
        for i in 2..=M {
            if X % i == 0 {
                ans = "No";
                break;
            }
        }
        println!("{}", ans);
    }
}