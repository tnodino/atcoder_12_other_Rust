// https://atcoder.jp/contests/abc126/tasks/abc126_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let A = S[..2].parse::<usize>().unwrap();
    let B = S[2..].parse::<usize>().unwrap();
    if (1 <= A && A <= 12) && (1 <= B && B <= 12) {
        println!("AMBIGUOUS");
    }
    else if 1 <= A && A <= 12 {
        println!("MMYY");
    }
    else if 1 <= B && B <= 12 {
        println!("YYMM");
    }
    else {
        println!("NA");
    }
}