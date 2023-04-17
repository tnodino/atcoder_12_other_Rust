// https://atcoder.jp/contests/past202107-open/tasks/past202107_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut res = 0;
    for i in (0..14).step_by(2) {
        res += S[i] as usize - 48;
    }
    res *= 3;
    for i in (1..14).step_by(2) {
        res += S[i] as usize - 48;
    }
    if res % 10 == S[14] as usize - 48 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}