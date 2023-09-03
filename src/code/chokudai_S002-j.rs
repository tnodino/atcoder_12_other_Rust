// https://atcoder.jp/contests/chokudai_S002/tasks/chokudai_S002_j

use proconio::input;
use libm::sqrt;

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
    let mut vec = Vec::new();
    let M = sqrt(A[0] as f64) as usize;
    for i in 1..=M {
        if A[0] % i == 0 {
            vec.push(i);
            vec.push(A[0] / i);
        }
    }
    let M = sqrt(B[0] as f64) as usize;
    for i in 1..=M {
        if B[0] % i == 0 {
            vec.push(i);
            vec.push(B[0] / i);
        }
    }
    vec.sort();
    vec.dedup();
    'outer: for i in (0..vec.len()).rev() {
        for j in 0..N {
            if A[j] % vec[i] != 0 && B[j] % vec[i] != 0 {
                continue 'outer;
            }
        }
        println!("{}", vec[i]);
        return;
    }
}