// https://atcoder.jp/contests/past202109-open/tasks/past202109_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize; N],
        B: [usize; M],
    }
    let mut vec = Vec::new();
    for i in 0..N {
        for j in 0..M {
            if A[i] == B[j] {
                vec.push(A[i]);
            }
        }
    }
    vec.sort();
    println!("{}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}