// https://atcoder.jp/contests/past202004-open/tasks/past202004_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        S.push(s);
    }
    for i in (0..N-1).rev() {
        for j in 1..2*N-2 {
            if S[i][j] == '#' {
                if S[i+1][j-1] == 'X' || S[i+1][j] == 'X' || S[i+1][j+1] == 'X' {
                    S[i][j] = 'X';
                }
            }
        }
    }
    for i in 0..N {
        println!("{}", S[i].iter().collect::<String>());
    }
}