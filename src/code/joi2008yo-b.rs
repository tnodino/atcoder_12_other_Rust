// https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_b

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
    let mut ans = 0;
    for i in 0..N-2 {
        if S[i] == 'J' && S[i+1] == 'O' && S[i+2] == 'I' {
            ans += 1;
        }
    }
    println!("{}", ans);
    let mut ans = 0;
    for i in 0..N-2 {
        if S[i] == 'I' && S[i+1] == 'O' && S[i+2] == 'I' {
            ans += 1;
        }
    }
    println!("{}", ans);
}