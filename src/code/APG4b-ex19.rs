// https://atcoder.jp/contests/APG4b/tasks/APG4b_cd

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 9;
    let mut correct = 0;
    let mut wrong = 0;
    for i in 1..=N {
        input! {
            A: [usize; N],
        }
        let mut ans = Vec::new();
        for j in 1..=N {
            ans.push(i * j);
            if i * j == A[j-1] {
                correct += 1;
            }
            else {
                wrong += 1;
            }
        }
        println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
    println!("{}", correct);
    println!("{}", wrong);
}