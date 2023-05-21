// https://atcoder.jp/contests/past202010-open/tasks/past202010_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        S: String,
    }
    let S = S.chars().collect::<Vec<char>>();
    let mut ansx = 100;
    let mut ansy = 100;
    for x in 0..N {
        for y in 0..N {
            let mut T = S.clone();
            for _ in 0..x {
                for i in 1..N {
                    if T[i] == '#' {
                        T[i-1] = '#';
                    }
                }
            }
            for _ in 0..y {
                for i in (0..N-1).rev() {
                    if T[i] == '#' {
                        T[i+1] = '#';
                    }
                }
            }
            T.dedup();
            if T.len() == 1 && x + y < ansx + ansy {
                ansx = x;
                ansy = y;
            }
        }
    }
    println!("{} {}", ansx, ansy);
}