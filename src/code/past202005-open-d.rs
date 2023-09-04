// https://atcoder.jp/contests/past202005-open/tasks/past202005_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    let num = [[['#', '#', '#'], ['#', '.', '#'], ['#', '.', '#'], ['#', '.', '#'], ['#', '#', '#']],
                [['.', '#', '.'], ['#', '#', '.'], ['.', '#', '.'], ['.', '#', '.'], ['#', '#', '#']],
                [['#', '#', '#'], ['.', '.', '#'], ['#', '#', '#'], ['#', '.', '.'], ['#', '#', '#']],
                [['#', '#', '#'], ['.', '.', '#'], ['#', '#', '#'], ['.', '.', '#'], ['#', '#', '#']],
                [['#', '.', '#'], ['#', '.', '#'], ['#', '#', '#'], ['.', '.', '#'], ['.', '.', '#']],
                [['#', '#', '#'], ['#', '.', '.'], ['#', '#', '#'], ['.', '.', '#'], ['#', '#', '#']],
                [['#', '#', '#'], ['#', '.', '.'], ['#', '#', '#'], ['#', '.', '#'], ['#', '#', '#']],
                [['#', '#', '#'], ['.', '.', '#'], ['.', '.', '#'], ['.', '.', '#'], ['.', '.', '#']],
                [['#', '#', '#'], ['#', '.', '#'], ['#', '#', '#'], ['#', '.', '#'], ['#', '#', '#']],
                [['#', '#', '#'], ['#', '.', '#'], ['#', '#', '#'], ['.', '.', '#'], ['#', '#', '#']]];
    input! {
        N: usize,
        s1: String,
        s2: String,
        s3: String,
        s4: String,
        s5: String,
    }
    let s1 = s1.chars().collect::<Vec<char>>();
    let s2 = s2.chars().collect::<Vec<char>>();
    let s3 = s3.chars().collect::<Vec<char>>();
    let s4 = s4.chars().collect::<Vec<char>>();
    let s5 = s5.chars().collect::<Vec<char>>();
    let mut ans = Vec::new();
    for i in (1..N*4).step_by(4) {
        let mut vec = Vec::new();
        vec.push(&s1[i..i+3]);
        vec.push(&s2[i..i+3]);
        vec.push(&s3[i..i+3]);
        vec.push(&s4[i..i+3]);
        vec.push(&s5[i..i+3]);
        for j in 0..10 {
            if vec == num[j] {
                ans.push(j);
            }
        }
    }
    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<String>());
}