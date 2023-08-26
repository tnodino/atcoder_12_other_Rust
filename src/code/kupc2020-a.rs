// https://atcoder.jp/contests/kupc2020/tasks/kupc2020_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        mut x: isize,
        mut y: isize,
    }
    let mut ans = 0;
    for _ in 0..N-1 {
        input! {
            nx: isize,
            ny: isize,
        }
        ans += (x - nx).abs() + (y - ny).abs();
        x = nx;
        y = ny;
    }
    println!("{}", ans);
}