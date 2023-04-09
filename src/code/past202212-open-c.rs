// https://atcoder.jp/contests/past202212-open/tasks/past202212_c

use proconio::input;
use proconio::fastout;
use std::collections::HashSet;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut set = HashSet::new();
    for i in 0..N {
        for j in i+1..N {
            for k in j+1..N {
                set.insert(A[i] * A[j] * A[k]);
            }
        }
    }
    println!("{}", set.len());
}