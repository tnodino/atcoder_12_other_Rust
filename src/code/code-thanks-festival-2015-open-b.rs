// https://atcoder.jp/contests/code-thanks-festival-2015-open/tasks/code_thanks_festival_2015_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        B1: usize,
        B2: usize,
        C: usize,
    }
    let mut ans = Vec::new();
    if A1 == C {
        ans.push(B1);
        ans.push(B2);
    }
    if A2 == C {
        ans.push(B1);
        ans.push(B2);
    }
    if B1 == C {
        ans.push(A1);
        ans.push(A2);
    }
    if B2 == C {
        ans.push(A1);
        ans.push(A2);
    }
    ans.sort();
    ans.dedup();
    let N = ans.len();
    println!("{}", N);
    for i in 0..N {
        println!("{}", ans[i]);
    }
}