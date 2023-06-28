// https://atcoder.jp/contests/past202212-open/tasks/past202212_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let mut stack = Vec::new();
    let mut N = 0;
    for s in S.chars() {
        stack.push(s);
        N += 1;
        while N >= 2 && stack[N-2] == '(' && stack[N-1] == ')' {
            stack.pop();
            stack.pop();
            N -= 2;
        }
    }
    if N > 0 {
        println!("No");
    }
    else {
        println!("Yes");
    }
}