// https://atcoder.jp/contests/APG4b/tasks/APG4b_cg

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        A1: usize,
        A2: usize,
        A3: usize,
        A4: usize,
        A5: usize,
    }
    let A = [A1, A2, A3, A4, A5];
    for i in 0..4 {
        if A[i] == A[i+1] {
            println!("YES");
            return;
        }
    }
    println!("NO");
}