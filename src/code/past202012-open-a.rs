// https://atcoder.jp/contests/past202012-open/tasks/past202012_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    if (S[0] == S[1] && S[1] == S[2]) || (S[1] == S[2] && S[2] == S[3]) || (S[2] == S[3] && S[3] == S[4]) {
        println!("{}", S[2]);
    }
    else {
        println!("draw");
    }
}