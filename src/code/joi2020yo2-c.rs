// https://atcoder.jp/contests/joi2020yo2/tasks/joi2020_yo2_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut cnt = vec![1; N+1];
    for i in 1..N {
        let mut s = 0;
        let mut x = i;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        if i + s <= N {
            cnt[i+s] += cnt[i];
        }
    }
    println!("{}", cnt[N]);
}