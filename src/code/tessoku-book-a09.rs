// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_i

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize,
    }
    let mut grid = vec![vec![0; W+2]; H+2];
    for _ in 0..N {
        input! {
            A: usize,
            B: usize,
            C: usize,
            D: usize,
        }
        grid[A][B] += 1;
        grid[A][D+1] -= 1;
        grid[C+1][B] -= 1;
        grid[C+1][D+1] += 1;
    }
    for i in 1..=H {
        for j in 1..=W {
            grid[i][j] += grid[i-1][j] + grid[i][j-1] - grid[i-1][j-1];
        }
    }
    for i in 1..=H {
        println!("{}", grid[i][1..=W].iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}