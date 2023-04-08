// https://atcoder.jp/contests/past201912-open/tasks/past201912_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    for i in 0..N-1 {
        if A[i] < A[i+1] {
            println!("up {}", A[i+1] - A[i]);
        }
        else if A[i] > A[i+1] {
            println!("down {}", A[i] - A[i+1]);
        }
        else {
            println!("stay");
        }
    }
}