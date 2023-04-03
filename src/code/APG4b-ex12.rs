// https://atcoder.jp/contests/APG4b/tasks/APG4b_ck

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut ans = 1;
    for i in (1..N).step_by(2) {
        ans += match S[i] {
            '+' => 1,
            _ => -1,
        };
    }
    println!("{}", ans);
}