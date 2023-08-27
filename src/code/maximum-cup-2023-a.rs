// https://atcoder.jp/contests/maximum-cup-2023/tasks/maximum_cup_2023_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        a: [usize; N],
    }
    let mut ans: usize = 0;
    let mut l = 0;
    for r in 1..N {
        if a[l] != a[r] {
            l = r;
        }
        ans += r - l;
    }
    println!("{}", ans);
}