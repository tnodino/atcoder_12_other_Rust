// https://atcoder.jp/contests/past202104-open/tasks/past202104_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut flg = vec![vec![false; M]; N];
    for i in 0..N {
        input! {
            K: usize,
            A: [usize; K],
        }
        for k in 0..K {
            flg[i][A[k]-1] = true;
        }
    }
    input! {
        P: usize,
        Q: usize,
        B: [usize; P],
    }
    let mut ans = 0;
    for i in 0..N {
        let mut cnt = 0;
        for p in 0..P {
            if flg[i][B[p]-1] {
                cnt += 1;
            }
        }
        if cnt >= Q {
            ans += 1;
        }
    }
    println!("{}", ans);
}