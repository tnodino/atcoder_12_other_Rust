// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bv

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut x_idx = vec![0; N+1];
    let mut y_idx = vec![0; N+1];
    for i in 0..N {
        input! {
            P: [usize; N],
        }
        for j in 0..N {
            x_idx[P[j]] = i;
            y_idx[P[j]] = j;
        }
    }
    let mut ans = 0;
    for i in 1..=N {
        for j in i+1..=N {
            if x_idx[i] > x_idx[j] {
                ans += 1;
            }
            if y_idx[i] > y_idx[j] {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}