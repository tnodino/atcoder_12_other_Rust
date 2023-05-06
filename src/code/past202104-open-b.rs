// https://atcoder.jp/contests/past202104-open/tasks/past202104_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    for i in (1..S.len()).step_by(4) {
        if S[i] == 'o' {
            println!("{}", i / 4 + 1);
            return;
        }
    }
    println!("none");
}