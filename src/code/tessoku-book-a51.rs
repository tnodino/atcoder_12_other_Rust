// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ay

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        Q: usize,
    }
    let mut stk = Vec::new();
    let mut cnt = 0;
    for _ in 0..Q {
        input! {
            q: usize,
        }
        if q == 1 {
            input! {
                x: String,
            }
            stk.push(x);
            cnt += 1;
        }
        else if q == 2 {
            println!("{}", stk[cnt-1]);
        }
        else {
            stk.pop();
            cnt -= 1;
        }
    }
}