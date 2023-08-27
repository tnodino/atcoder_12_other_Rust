// https://atcoder.jp/contests/past202212-open/tasks/past202212_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = vec![0; N];
    let mut cnt = 0;
    for i in 0..M {
        match S[i] {
            '+' => {
                ans[i%N] += cnt + 1;
                cnt = 0;
            },
            '-' => {
                cnt += ans[i%N] + 1;
                ans[i%N] = 0;
            },
            _ => ans[i%N] += 1,
        }
    }
    for i in 0..N {
        println!("{}", ans[i]);
    }
}