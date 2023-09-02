// https://atcoder.jp/contests/mujin-pc-2018/tasks/mujin_pc_2018_a

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    if S.len() >= 5 && &S[..5] == "MUJIN" {
        println!("Yes");
    }
    else {
        println!("No");
    }
}