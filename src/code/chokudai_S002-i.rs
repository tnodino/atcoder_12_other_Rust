// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_i

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
        }
        A.push(a);
        B.push(b);
    }
    let mut i = 0;
    for j in 1..N {
        if (A[i] + B[j] - 1) / B[j] < (A[j] + B[i] - 1) / B[i] {
            i = j;
        }
    }
    for j in 0..N {
        if i == j {
            continue;
        }
        if (A[i] + B[j] - 1) / B[j] <= (A[j] + B[i] - 1) / B[i] {
            println!("-1");
            return;
        }
    }
    println!("{}", i + 1);
}