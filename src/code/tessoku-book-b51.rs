// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_dx

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut stk = Vec::new();
    for i in 0..N {
        if S[i] == '(' {
            stk.push(i+1);
        }
        else {
            println!("{} {}", stk.pop().unwrap(), i+1);
        }
    }
}