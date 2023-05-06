// https://atcoder.jp/contests/past202112-open/tasks/past202112_c

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut flg = [0; 6];
    for i in 0..N {
        input! {
            P: char,
            V: String,
        }
        if V == "WA" {
            continue;
        }
        let idx = match P {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            _ => 5,
        };
        if flg[idx] == 0 {
            flg[idx] = i + 1;
        }
    }
    for i in 0..6 {
        println!("{}", flg[i]);
    }
}