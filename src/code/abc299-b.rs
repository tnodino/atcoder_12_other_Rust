// https://atcoder.jp/contests/abc299/tasks/abc299_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut T: usize,
        C: [usize; N],
        R: [usize; N],
    }
    if !C.contains(&T) {
        T = C[0];
    }
    let mut ans = 0;
    let mut ma = 0;
    for i in 0..N {
        if C[i] == T && R[i] > ma {
            ans = i;
            ma = R[i
            ];
        }
    }
    println!("{}", ans + 1);
}