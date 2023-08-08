// https://atcoder.jp/contests/joisc2007/tasks/joisc2007_anagra

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        S: String,
    }
    let N = S.len();
    let S = S.chars().collect::<Vec<char>>();
    let mut cnt = [0; 26];
    for i in 0..N {
        let idx = (S[i] as usize) - ('A' as usize);
        cnt[idx] += 1;
    }
    let mut perm: Vec<usize> = vec![0; N+1];
    perm[0] = 1;
    for i in 1..=N {
        perm[i] = perm[i-1] * i;
    }
    let mut ans = 1;
    for i in 0..N-1 {
        for j in 0..26 {
            let idx = (S[i] as usize) - ('A' as usize);
            if idx == j {
                cnt[j] -= 1;
                break;
            }
            let mut res = perm[N-i-1] * cnt[j];
            for k in 0..26 {
                res /= perm[cnt[k]];
            }
            ans += res;
        }
    }
    println!("{}", ans);
}