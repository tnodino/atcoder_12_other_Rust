// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_ff

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 10000;
    let mut flg = vec![true; M];
    for _ in 0..N {
        input! {
            mut S: usize,
            T: usize,
        }
        let mut s = Vec::new();
        for _ in 0..4 {
            s.push(S % 10);
            S /= 10;
        }
        for i in 0..M {
            let mut num = i;
            let mut cnt = 0;
            for j in 0..4 {
                if num % 10 != s[j] {
                    cnt += 1;
                }
                num /= 10;
            }
            let res = match T {
                1 => if cnt == 0 {
                    true
                }
                else {
                    false
                },
                2 => if cnt == 1 {
                    true
                }
                else {
                    false
                }
                _ => if cnt >= 2 {
                    true
                }
                else {
                    false
                }
            };
            flg[i] &= res;
        }
    }
    if flg.iter().filter(|&x| *x).count() >= 2 {
        println!("Can't Solve");
    }
    else {
        for i in 0..M {
            if flg[i] {
                println!("{:04}", i);
                return;
            }
        }
    }
}