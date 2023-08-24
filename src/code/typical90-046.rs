// https://atcoder.jp/contests/typical90/tasks/typical90_at

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
    }
    let M = 46;
    let mut cnta = vec![0; M];
    let mut cntb = vec![0; M];
    let mut cntc = vec![0; M];
    for i in 0..N {
        cnta[A[i]%M] += 1;
        cntb[B[i]%M] += 1;
        cntc[C[i]%M] += 1;
    }
    let mut ans: usize = 0;
    for i in 0..M {
        for j in 0..M {
            for k in 0..M {
                if (i + j + k) % M == 0 {
                    ans += cnta[i] * cntb[j] * cntc[k];
                }
            }
        }
    }
    println!("{}", ans);
}