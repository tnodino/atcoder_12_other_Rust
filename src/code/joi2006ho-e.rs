// https://atcoder.jp/contests/joi2006ho/tasks/joi2006ho_e

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        r: usize,
    }
    let m = 10010;
    let mut vec = Vec::new();
    for _ in 0..n {
        input! {
            x1: usize,
            y1: usize,
            x2: usize,
            y2: usize,
        }
        vec.push((x1+1, y1+1, y2+1, 1));
        vec.push((x2+1, y1+1, y2+1, 2));
    }
    vec.sort();
    let mut cnt1 = vec![0; m];
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut idx = 0;
    for i in 1..m {
        let mut cnt2 = vec![0; m];
        while idx < n * 2 && vec[idx].0 == i {
            if vec[idx].3 == 1 {
                cnt2[vec[idx].1] += 1;
                cnt2[vec[idx].2] -= 1;
            }
            else {
                cnt2[vec[idx].1] -= 1;
                cnt2[vec[idx].2] += 1;
            }
            idx += 1;
        }
        for k in 1..m {
            cnt2[k] += cnt2[k-1] + cnt1[k] - cnt1[k-1];
            if cnt2[k] >= 1 {
                ans1 += 1;
                if cnt2[k-1] == 0 {
                    ans2 += 2;
                }
                if cnt1[k] == 0 {
                    ans2 += 2;
                }
            }
        }
        cnt1 = cnt2;
    }
    println!("{}", ans1);
    if r == 2 {
        println!("{}", ans2);
    }
}