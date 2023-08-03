// https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_bh

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut ans = Vec::new();
    let mut stc = Vec::new();
    for i in 0..N {
        let mut res = -1;
        while !stc.is_empty() {
            let idx = stc.last().unwrap();
            if A[*idx-1] > A[i] {
                res = *idx as isize;
                break;
            }
            stc.pop();
        }
        ans.push(res);
        stc.push(i+1);
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}