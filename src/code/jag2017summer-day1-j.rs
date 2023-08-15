// https://atcoder.jp/contests/jag2017summer-day1/tasks/jag2017summer_day1_j

use proconio::fastout;
use libm::sqrt;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let N = 98;
    let mut ans = Vec::new();
    for i in 1..=N {
        let mut res = 0;
        let M = sqrt(i as f64) as usize + 1;
        for j in 1..=M {
            if i % j == 0 {
                res += j;
                if i / j != j {
                    res += i / j;
                }
            }
        }
        if res == 98 {
            ans.push(i);
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}