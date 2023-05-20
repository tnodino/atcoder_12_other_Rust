// https://atcoder.jp/contests/past202012-open/tasks/past202012_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
    }
    if N == 0 {
        println!("0");
        return;
    }
    let num = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let mut ans = "".to_string();
    while N > 0 {
        let x = N % 36;
        ans = format!("{}{}", num[x], ans);
        N /= 36;
    }
    println!("{}", ans);
}