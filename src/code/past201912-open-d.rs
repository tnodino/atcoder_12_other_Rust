// https://atcoder.jp/contests/past201912-open/tasks/past201912_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut cnt = vec![0; N];
    for i in 0..N {
        cnt[A[i]-1] += 1;
    }
    if !cnt.contains(&2) {
        println!("Correct");
        return
    }
    let mut x = 0;
    let mut y = 0;
    for i in 0..N {
        if cnt[i] == 0 {
            x = i + 1;
        }
        if cnt[i] == 2 {
            y = i + 1;
        }
    }
    println!("{} {}", y, x);
}