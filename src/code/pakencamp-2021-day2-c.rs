// https://atcoder.jp/contests/pakencamp-2021-day2/tasks/pakencamp_2021_day2_c

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
    let L = 3000;
    let mut flg = vec![false; L];
    for i in 0..N {
        flg[A[i]-1] = true;
    }
    let mut ans = Vec::new();
    for i in 0..M {
        if !flg[B[i]-1] {
            ans.push(B[i]);
        }
    }
    println!("{}", ans.len());
    for a in ans.iter() {
        println!("{}", a);
    }
}