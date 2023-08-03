// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_y

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
    }
    let mut c = Vec::new();
    for _ in 0..H {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut DP: Vec<Vec<usize>> = vec![vec![0; W]; H];
    DP[0][0] = 1;
    for i in 0..H {
        for j in 0..W {
            if c[i][j] == '#' {
                continue;
            }
            if 0 < i {
                DP[i][j] += DP[i-1][j];
            }
            if 0 < j {
                DP[i][j] += DP[i][j-1];
            }
        }
    }
    println!("{}", DP[H-1][W-1]);
}