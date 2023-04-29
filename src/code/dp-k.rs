// https://atcoder.jp/contests/dp/tasks/dp_k

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        a: [usize; N],
    }
    let mut DP = vec![false; K+1];
    for i in 0..=K {
        let mut flg = false;
        for j in 0..N {
            if a[j] <= i && !DP[i-a[j]]{
                flg = true;
            }
        }
        DP[i] = flg;
    }
    if DP[K] {
        println!("First");
    }
    else {
        println!("Second");
    }
}